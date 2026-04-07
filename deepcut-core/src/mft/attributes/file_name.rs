use crate::errors::DeepcutError;
use crate::mft::attributes::{Attribute, StandardInformation};
use crate::mft::errors::MftError;
use crate::mft::errors::MftError::MftResidentAttributeParseError;
use crate::utils;

#[derive(Clone)]
pub struct FileName {
    pub ref_to_parent: u64,
    pub ctime: u64,
    pub atime: u64,
    pub mtime: u64,
    pub rtime: u64,
    pub allocated_size: u64,
    pub real_size: u64,
    pub flags: u32,
    pub eas_and_reparse: u32,
    pub filename_len_in_chars: u8,
    pub filename_space: u8,
    pub name: Option<String>,
}

impl FileName {
    pub fn parse(buf: &[u8]) -> Result<Self, DeepcutError> {
        if buf.len() < 8 {
            return Err(DeepcutError::from(MftError::MftAttributeStandardInformationSmallBuffer));
        }

        let filename_len_in_chars: u8 = utils::read_bytes(&buf[64..65])?;
        let name = utils::extract_string(&buf[66..], filename_len_in_chars as usize);

        Ok(Self {
            ref_to_parent: utils::read_bytes(&buf[0..8])?,
            ctime: utils::read_bytes(&buf[8..16])?,
            atime: utils::read_bytes(&buf[16..24])?,
            mtime: utils::read_bytes(&buf[24..32])?,
            rtime: utils::read_bytes(&buf[32..40])?,
            allocated_size: utils::read_bytes(&buf[40..48])?,
            real_size: utils::read_bytes(&buf[48..56])?,
            flags: utils::read_bytes(&buf[56..60])?,
            eas_and_reparse: utils::read_bytes(&buf[60..64])?,
            filename_len_in_chars,
            filename_space: utils::read_bytes(&buf[65..66])?,
            name,
        })
    }

    pub fn get(attribute: Attribute) -> Option<FileName> {
        match attribute {
            Attribute::FileName(file_name) => Some(file_name),
            _ => None,
        }
    }
}