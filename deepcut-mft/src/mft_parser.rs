use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use deepcut_core::mft::structure::{MftHeader, MFT_RECORD_SIZE};
use deepcut_core::errors;
use deepcut_core::errors::{CoreError, ERROR};
use deepcut_core::mft::errors::MftError;
use crate::vprintln;

pub fn parse(path: &str) {
    let mut file = match read_file(path) {
        Ok(contents) => contents,
        Err(e) => {
            vprintln!("Could not read file {}: {}", path, e);
            return;
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

    loop {
        if file_length != 0 && pointer + MFT_RECORD_SIZE > file_length {
            break;
        }

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
                if e == ERROR::from(MftError::MftHeaderSignatureBad) {
                    corrupted_records += 1;
                } else if e == ERROR::from(MftError::MftHeaderSignatureInvalid) {
                    invalid_records += 1;
                }
                continue;
            }
        };
        good_records += 1;

        if !fix_usn(&mut record, mft_header.usa_offset as usize) {
            continue;
        }
    }

    vprintln!("Good record count: {}\nCorrupted records: {}\nInvalid records: {}", good_records, corrupted_records, invalid_records);
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

    return true;
}

fn read_contents(file: &mut File, start: usize, end: usize) -> Result<Vec<u8>, MftError> {
    match file.seek(SeekFrom::Start(start as u64)) {
        Ok(_) => (),
        Err(e) => {
            vprintln!("Failed to seek file:\n{e}");
            return Err(MftError::MftRecordFailedToGetBytesFromRecord);
        }
    }

    let mut buffer = vec![start as u8; end - start];

    match file.read_exact(&mut buffer) {
        Ok(_) => { Ok(buffer) },
        Err(e) => {
            vprintln!("Failed to read exact:\n{e}");
            Err(MftError::MftRecordFailedToGetBytesFromRecord)
        }
    }
}

fn read_file(path: &str) -> Result<File, &'static str> {
    vprintln!("Parsing {}", path);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            vprintln!("Could not open file: {}", e);
            return Err("Could not open file");
        }
    };

    Ok(file)
}