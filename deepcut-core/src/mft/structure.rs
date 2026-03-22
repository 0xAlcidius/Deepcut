use crate::errors::{DeepcutError};
use crate::mft::errors::MftError;
use crate::utils;
const MFT_SIGNATURE: u32 = 0x46494C45;
const MFT_BAD: u32 = 0x42414144;
const MFT_HEADER_SIZE: usize = 0x30;
pub const MFT_RECORD_SIZE: usize = 0x400;
pub const MFT_ATTRIBUTE_MAX_SIZE: usize = 0x40;

// Unique Sequence Array (USA)
// Logfile Sequence Number (LSN)
pub struct MftHeader {
    pub usa_offset: u16,
    pub usa_count: u16,
    pub lsn: u64,
    pub seq_num: u16,
    pub hard_link_count: u16,
    pub attr_offset: u16,
    pub flags: u16,
    pub used_size: u32,
    pub alloc_size: u32,
    pub base_ref: u64,
    pub next_attr_id: u16,
    pub record_number: u32,
}

// Source: https://flatcap.github.io/linux-ntfs/ntfs/concepts/attribute_header.html
pub struct MftAttributeHeader {
    pub attr_type: u32,
    pub attr_len: u32,
    pub non_resident: u8,
    pub name_len: u8,
    pub name_offset: u16,
    pub flags: u16,
    pub attr_id: u16,
    pub name: Option<String>,
    pub data: Option<AttributeData>,
}

pub enum AttributeData {
    Resident(MftResidentAttribute),
    NonResident(MftNonResidentAttribute),
}

pub struct MftResidentAttribute {
    pub content_len: u32,
    pub attr_offset: u16,
    pub index_flags: u8,
}

// Compression Unit Size (CUS)
pub struct MftNonResidentAttribute {
    pub start_vcn: u64,
    pub last_vcn: u64,
    pub data_runs_offset: u16,
    pub cus: u16,
    pub alloc_size: u64,
    pub actual_attr: u64,
    pub init_data_size: u64,
}


impl MftHeader {
    pub fn parse(buf: &[u8]) -> Result<Self, DeepcutError> {
        if buf.len() < MFT_HEADER_SIZE {
            return Err(DeepcutError::from(MftError::MftRecordTooSmall));
        }

        let signature = match buf[..4].try_into() {
            Ok(signature) => u32::from_be_bytes(signature),
            Err(_) => return Err(DeepcutError::from(MftError::MftRecordFailedToGetBytesFromRecord)),
        };

        if signature != MFT_SIGNATURE {
            if signature == MFT_BAD {
                return Err(DeepcutError::from(MftError::MftHeaderSignatureBad));
            }
            return Err(DeepcutError::from(MftError::MftHeaderSignatureInvalid));
        }

        let usa_offset: u16 = match utils::read_bytes(&buf[4..6]) {
            Ok(usa_offset) => usa_offset,
            Err(e) => return Err(DeepcutError::from(e)),
        };

        let usa_count: u16 = match utils::read_bytes(&buf[6..8]) {
            Ok(usa_count) => usa_count,
            Err(e) => return Err(DeepcutError::from(e)),
        };

        Ok(Self {
            usa_offset,
            usa_count,
            lsn:             utils::read_bytes(&buf[8..16])?,
            seq_num:         utils::read_bytes(&buf[16..18])?,
            hard_link_count: utils::read_bytes(&buf[18..20])?,
            attr_offset:     utils::read_bytes(&buf[20..22])?,
            flags:           utils::read_bytes(&buf[22..24])?,
            used_size:       utils::read_bytes(&buf[24..28])?,
            alloc_size:      utils::read_bytes(&buf[28..32])?,
            base_ref:        utils::read_bytes(&buf[32..40])?,
            next_attr_id:    utils::read_bytes(&buf[40..42])?,
            record_number:   utils::read_bytes(&buf[44..48])?,
        })
    }
}

impl MftAttributeHeader {
    pub fn parse(buf: &[u8]) -> Result<Self, DeepcutError> {
        let name_len: u8 = utils::read_bytes(&buf[9..10])?;

        let name = if name_len == 0 {
            None
        } else {
            let start_u16: u16 = utils::read_bytes(&buf[10..12])?;

            let start: usize = start_u16 as usize;
            let end = start + (name_len as usize * 2);

            if start >= buf.len() || end > buf.len() {
                None
            } else {
                let name_bytes = &buf[start..end];

                let utf16: Vec<u16> = name_bytes
                    .chunks_exact(2)
                    .map(|b| u16::from_le_bytes([b[0], b[1]]))
                    .collect();

                String::from_utf16(&utf16).ok()
            }
        };

        let non_resident: u8 = utils::read_bytes(&buf[8..9])?;

        let attr_data = if non_resident == 0 {
            AttributeData::Resident(MftResidentAttribute::parse(&buf[16..24])?)
        } else {
            AttributeData::NonResident(MftNonResidentAttribute::parse(&buf[16..64])?)
        };

        Ok(Self {
            attr_type:    utils::read_bytes(&buf[..4])?,
            attr_len:     utils::read_bytes(&buf[4..8])?,
            non_resident,
            name_len,
            name_offset:  utils::read_bytes(&buf[10..12])?,
            flags:        utils::read_bytes(&buf[12..14])?,
            attr_id:      utils::read_bytes(&buf[14..16])?,
            name,
            data:         Some(attr_data),
        })
    }
}

impl MftResidentAttribute {
    pub fn parse(buf: &[u8]) -> Result<Self, DeepcutError> {
        Ok(Self {
            content_len: utils::read_bytes(&buf[0..4])?,
            attr_offset: utils::read_bytes(&buf[4..6])?,
            index_flags: utils::read_bytes(&buf[6..7])?,
        })
    }
}

impl MftNonResidentAttribute {
    pub fn parse(buf: &[u8]) -> Result<Self, DeepcutError> {
        Ok(Self {
            start_vcn:        utils::read_bytes(&buf[0..8])?,
            last_vcn:         utils::read_bytes(&buf[8..16])?,
            data_runs_offset: utils::read_bytes(&buf[16..18])?,
            cus:              utils::read_bytes(&buf[18..20])?,
            alloc_size:       utils::read_bytes(&buf[24..32])?,
            actual_attr:      utils::read_bytes(&buf[32..40])?,
            init_data_size:   utils::read_bytes(&buf[40..48])?,
        })
    }
}