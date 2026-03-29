use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use deepcut_core::mft::structure::{AttributeData, MftAttributeHeader, MftHeader, MFT_ATTRIBUTE_MAX_SIZE, MFT_RECORD_SIZE};
use deepcut_core::errors::{DeepcutError};
use deepcut_core::mft::attributes;
use deepcut_core::mft::attributes::{standard_information::StandardInformation, file_name::FileName, Attribute, standard_information};
use deepcut_core::mft::errors::MftError;
use crate::vprintln;

pub struct MftEntry {
    pub header: MftHeader,
    pub attributes: HashMap<usize, MftAttributeHeader>,
    pub standard_information: Option<StandardInformation>,
    pub file_name: Option<FileName>,
}

pub fn parse(path: &str) -> Result<HashMap<u64, MftEntry>, MftError> {
    let mut file = match File::open(path) {
        Ok(contents) => contents,
        Err(e) => {
            vprintln!("Could not read file {}: {}", path, e);
            return Err(MftError::MftFileOpenError);
        }
    };

    let mut good_records = 0;
    let mut corrupted_records = 0;
    let mut invalid_records = 0;
    let mut pointer = 0;

    let file_length: usize = match file.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(e) => {
            vprintln!("Failed to retrieve metadata of file {}",  e);
            0
        }
    };

    let mut results: HashMap<u64, MftEntry> = HashMap::new();
    loop {
        if file_length != 0 && pointer + MFT_RECORD_SIZE > file_length {
            break;
        }
        let start = pointer;
        let mut record = match read_contents(&mut file, pointer, pointer + MFT_RECORD_SIZE) {
            Ok(contents) => contents,
            Err(e) => {
                vprintln!("Failed to read MFT record at {pointer}, exiting early.",);
                break;
            }
        };
        pointer += MFT_RECORD_SIZE;

        let mft_header = match MftHeader::parse(&record) {
            Ok(header) => header,
            Err(e) => {
                if e == DeepcutError::from(MftError::MftHeaderSignatureBad) {
                    corrupted_records += 1;
                } else if e == DeepcutError::from(MftError::MftHeaderSignatureInvalid) {
                    invalid_records += 1;
                }
                continue;
            }
        };
        good_records += 1;

        if !fix_usn(&mut record, mft_header.usa_offset as usize) {
            continue;
        }

        let attributes_map = parse_record_attributes(&record, mft_header.attr_offset as usize);

        let mut unknown_attributes = Vec::new();
        for k in attributes_map.keys() {
            let offset = *k;
            let mft_attribute = match attributes_map.get(k) {
                Some(attribute) => attribute,
                None => continue,
            };

            let content_result = match &mft_attribute.data {
                Some(AttributeData::Resident(resident)) => {
                    let start = offset + resident.attr_offset as usize;
                    let end = start + resident.content_len as usize;
                    if start > MFT_RECORD_SIZE || end > MFT_RECORD_SIZE {
                        vprintln!("Failed to parse attribute type");
                        continue;
                    }
                    Ok(&record[start..end])
                },
                Some(AttributeData::NonResident(non_resident)) => {
                    //todo!("Implement NonResident");
                    continue;
                },
                None => Err("Failed to parse attribute type"),
            };

            let content = match content_result {
                Ok(content) => content,
                Err(e) => {
                    continue;
                }
            };

            let unknown_attribute = match parse_resident_attribute(mft_attribute, content) {
                Some(a) => a,
                None => continue,
            };

            unknown_attributes.push(unknown_attribute);
        }
        let mut si: Option<StandardInformation> = None;
        let mut file_name: Option<FileName> = None;

        for attr in unknown_attributes {
            si = StandardInformation::get(attr.clone());
            file_name = FileName::get(attr);
        }

        results.insert(start as u64, MftEntry {
            header: mft_header,
            attributes: attributes_map,
            standard_information: si,
            file_name,
        });
    }

    vprintln!("Good record count: {}\nCorrupted records: {}\nInvalid records: {}", good_records, corrupted_records, invalid_records);
    Ok(results)
}

fn parse_resident_attribute(attribute: &MftAttributeHeader, buf: &[u8]) -> Option<Attribute> {
    match attribute.attr_type {
        0x10 => {
            match StandardInformation::parse(&buf) {
                Ok(si) => Some(Attribute::StandardInformation(si)),
                Err(e) => {
                    vprintln!("Failed to parse SI");
                    None
                }
            }
        },
        0x30 => {
            match FileName::parse(&buf) {
                Ok(file_name) => Some(Attribute::FileName(file_name)),
                Err(e) => {
                    vprintln!("Failed to parse FileName");
                    None
                }
            }
        }
        _ => {None}
    }
}

fn parse_record_attributes(buf: &[u8], offset: usize) -> HashMap<usize, MftAttributeHeader> {
    let mut attributes = HashMap::new();
    let mut pointer = offset;
    loop {
        if pointer + MFT_ATTRIBUTE_MAX_SIZE > 0x400 {
            break;
        }

        let attr = match MftAttributeHeader::parse(&buf[pointer..pointer + MFT_ATTRIBUTE_MAX_SIZE]) {
            Ok(header) => header,
            Err(e) => {
                vprintln!("Error reading MFT attribute Header: {e}");
                continue;
            }
        };

        if attr.attr_type == 0xFFFFFFFF || attr.attr_len == 0 {
            break;
        }
        let attr_len = attr.attr_len as usize;
        attributes.insert(pointer, attr);
        pointer += attr_len;
    }
    attributes
}

fn fix_usn(buf: &mut [u8], usa: usize) -> bool {
    let usn: [u8; 2] = match buf[usa..usa+2].try_into() {
        Ok(usa) => usa,
        Err(e) => {
            vprintln!("Failed to convert bytes to usa: {}", e);
            return false;
        }
    };

    let usn_1: [u8; 2] = match buf[usa+2..usa+4].try_into() {
        Ok(usa) => usa,
        Err(e) => {
            vprintln!("Failed to convert bytes to usa: {}", e);
            return false;
        }
    };

    let usn_2: [u8; 2] = match buf[usa+4..usa+6].try_into() {
        Ok(usa) => usa,
        Err(e) => {
            vprintln!("Failed to convert bytes to usa: {}", e);
            return false;
        }
    };

    if usn == buf[510..512] {
        buf[510..512].copy_from_slice(&usn_1);
    } else {
        vprintln!("Bad usa value: {}\nFailed USN 1", usa);
        return false;
    }

    if usn == buf[1022..1024] {
        buf[1022..1024].copy_from_slice(&usn_2);
    } else {
        vprintln!("Bad usa value: {}\nFailed USN 2", usa);
        return false;
    }

    true
}

fn read_contents(file: &mut File, start: usize, end: usize) -> Result<Vec<u8>, MftError> {
    match file.seek(SeekFrom::Start(start as u64)) {
        Ok(_) => (),
        Err(e) => {
            vprintln!("Failed to seek file:\n{e}");
            return Err(MftError::MftRecordFailedToGetBytesFromRecord);
        }
    }

    let mut buffer = vec![0u8; end - start];

    match file.read_exact(&mut buffer) {
        Ok(_) => { Ok(buffer) },
        Err(e) => {
            vprintln!("Failed to read exact:\n{e}");
            Err(MftError::MftRecordFailedToGetBytesFromRecord)
        }
    }
}