use crate::errors::ERROR;
use crate::mft::errors::MftError;
use crate::utils;
const MFT_SIGNATURE: u32 = 0x46494C45;
const MFT_BAD: u32 = 0x42414144;
const MFT_HEADER_SIZE: usize = 0x30;
pub const MFT_RECORD_SIZE: usize = 0x400;

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
    pub fn parse(buf: &[u8]) -> Result<Self, ERROR> {
        if buf.len() < MFT_HEADER_SIZE {
            return Err(ERROR::from(MftError::MftRecordTooSmall));
        }

        let signature = match buf[..4].try_into() {
            Ok(signature) => u32::from_be_bytes(signature),
            Err(_) => return Err(ERROR::from(MftError::MftRecordFailedToGetBytesFromRecord)),
        };

        if signature != MFT_SIGNATURE {
            if signature == MFT_BAD {
                return Err(ERROR::from(MftError::MftHeaderSignatureBad));
            }
            return Err(ERROR::from(MftError::MftHeaderSignatureInvalid));
        }

        let usa_offset: u16 = match utils::read_bytes(&buf[4..6]) {
            Ok(usa_offset) => usa_offset,
            Err(e) => return Err(ERROR::from(e)),
        };

        let usa_count: u16 = match utils::read_bytes(&buf[6..8]) {
            Ok(usa_count) => usa_count,
            Err(e) => return Err(ERROR::from(e)),
        };

        let lsn: u64 = match utils::read_bytes(&buf[8..16]) {
            Ok(lsn) => lsn,
            Err(e) => return Err(ERROR::from(e)),
        };

        let seq_num: u16 = match utils::read_bytes(&buf[16..18]) {
            Ok(seq_num) => seq_num,
            Err(e) => return Err(ERROR::from(e)),
        };

        let hard_link_count: u16 = match utils::read_bytes(&buf[18..20]) {
            Ok(hard_link_count) => hard_link_count,
            Err(e) => return Err(ERROR::from(e)),
        };

        let attr_offset: u16 = match utils::read_bytes(&buf[20..22]) {
            Ok(attr_offset) => attr_offset,
            Err(e) => return Err(ERROR::from(e)),
        };

        let flags: u16 = match utils::read_bytes(&buf[22..24]) {
            Ok(flags) => flags,
            Err(e) => return Err(ERROR::from(e)),
        };

        let used_size: u32 = match utils::read_bytes(&buf[24..28]) {
            Ok(used_size) => used_size,
            Err(e) => return Err(ERROR::from(e)),
        };

        let alloc_size: u32 = match utils::read_bytes(&buf[28..32]) {
            Ok(alloc_size) => alloc_size,
            Err(e) => return Err(ERROR::from(e)),
        };

        let base_ref: u64 = match utils::read_bytes(&buf[32..40]) {
            Ok(base_ref) => base_ref,
            Err(e) => return Err(ERROR::from(e)),
        };

        let next_attr_id: u16 = match utils::read_bytes(&buf[40..42]) {
            Ok(next_attr_id) => next_attr_id,
            Err(e) => return Err(ERROR::from(e)),
        };

        let record_number: u32 = match utils::read_bytes(&buf[44..48]) {
            Ok(record_number) => record_number,
            Err(e) => return Err(ERROR::from(e)),
        };

        Ok(Self {
            usa_offset,
            usa_count,
            lsn,
            seq_num,
            hard_link_count,
            attr_offset,
            flags,
            used_size,
            alloc_size,
            base_ref,
            next_attr_id,
            record_number,
        })

        // Ok(Self {
        //     usa_offset: u16::from_le_bytes(buf[4..6].try_into().unwrap()),
        //     usa_count: u16::from_le_bytes(buf[6..8].try_into().unwrap()),
        //     lsn: u64::from_le_bytes(buf[8..16].try_into().unwrap()),
        //     seq_num: u16::from_le_bytes(buf[16..18].try_into().unwrap()),
        //     hard_link_count: u16::from_le_bytes(buf[18..20].try_into().unwrap()),
        //     attr_offset: u16::from_le_bytes(buf[20..22].try_into().unwrap()),
        //     flags: u16::from_le_bytes(buf[22..24].try_into().unwrap()),
        //     used_size: u32::from_le_bytes(buf[24..28].try_into().unwrap()),
        //     alloc_size: u32::from_le_bytes(buf[28..32].try_into().unwrap()),
        //     base_ref: u64::from_le_bytes(buf[32..40].try_into().unwrap()),
        //     next_attr_id: u16::from_le_bytes(buf[40..42].try_into().unwrap()),
        //     record_number: u32::from_le_bytes(buf[44..48].try_into().unwrap()),
        // })
    }
}
//
// impl MftAttributeHeader {
//     pub fn parse(buf: &[u8]) -> Result<Self, &'static str> {
//         let name_len = u8::from_le_bytes(buf[9..10].try_into().unwrap());
//
//         let name = if name_len == 0 {
//             None
//         } else {
//             let start = u16::from_le_bytes(buf[10..12].try_into().unwrap()) as usize;
//             let end = start + (name_len as usize * 2);
//
//             if start >= buf.len() || end > buf.len() {
//                 None
//             } else {
//                 let name_bytes = &buf[start..end];
//
//                 let utf16: Vec<u16> = name_bytes
//                     .chunks_exact(2)
//                     .map(|b| u16::from_le_bytes([b[0], b[1]]))
//                     .collect();
//
//                 String::from_utf16(&utf16).ok()
//             }
//         };
//
//         let non_resident = u8::from_le_bytes(buf[8..9].try_into().unwrap());
//
//         let attr_data = if non_resident == 0 {
//             AttributeData::Resident(MftResidentAttribute::parse(&buf[16..24]).unwrap())
//         } else {
//             AttributeData::NonResident(MftNonResidentAttribute::parse(&buf[16..64]).unwrap())
//         };
//
//         Ok(Self {
//             attr_type: u32::from_le_bytes(buf[0..4].try_into().unwrap()),
//             attr_len: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
//             non_resident: u8::from_le_bytes(buf[8..9].try_into().unwrap()),
//             name_len: u8::from_le_bytes(buf[9..10].try_into().unwrap()),
//             name_offset: u16::from_le_bytes(buf[10..12].try_into().unwrap()),
//             flags: u16::from_le_bytes(buf[12..14].try_into().unwrap()),
//             attr_id: u16::from_le_bytes(buf[14..16].try_into().unwrap()),
//             name,
//             data: Some(attr_data),
//         })
//     }
// }
//
// impl MftResidentAttribute {
//     pub fn parse(buf: &[u8]) -> Result<Self, &'static str> {
//         Ok(Self {
//             content_len: u32::from_le_bytes(buf[..4].try_into().unwrap()),
//             attr_offset: u16::from_le_bytes(buf[4..6].try_into().unwrap()),
//             index_flags: u8::from_le_bytes(buf[6..7].try_into().unwrap()),
//         })
//     }
// }
//
// impl MftNonResidentAttribute {
//     pub fn parse(buf: &[u8]) -> Result<Self, &'static str> {
//         Ok(Self {
//             start_vcn: u64::from_le_bytes(buf[..8].try_into().unwrap()),
//             last_vcn: u64::from_le_bytes(buf[8..16].try_into().unwrap()),
//             data_runs_offset: u16::from_le_bytes(buf[16..18].try_into().unwrap()),
//             cus: u16::from_le_bytes(buf[18..20].try_into().unwrap()),
//             alloc_size: u64::from_le_bytes(buf[24..32].try_into().unwrap()),
//             actual_attr: u64::from_le_bytes(buf[32..40].try_into().unwrap()),
//             init_data_size: u64::from_le_bytes(buf[40..48].try_into().unwrap()),
//         })
//     }
// }