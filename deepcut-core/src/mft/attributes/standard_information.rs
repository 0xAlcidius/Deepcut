use crate::errors::DeepcutError;
use crate::mft::attributes::Attribute;
use crate::mft::errors::MftError;
use crate::utils;

#[derive(Clone)]
pub struct StandardInformation {
    pub file_creation: u64,
    pub file_altered: u64,
    pub mft_changed: u64,
    pub file_read: u64,
    pub file_permissions: u32,
    pub max_num_of_versions: u32,
    pub version_number: u32,
    pub class_id: u32,
    pub owner_id: u32,
    pub security_id: u32,
    pub quota_charged: u64,
    pub update_sequence_number: u64,
}

impl StandardInformation {
    pub fn parse(buf: &[u8]) -> Result<Self, DeepcutError> {
        if buf.len() < 0x42 {
            return Err(DeepcutError::from(MftError::MftAttributeStandardInformationSmallBuffer));
        }

        Ok(Self {
            file_creation:          utils::read_bytes(&buf[0..8])?,
            file_altered:           utils::read_bytes(&buf[8..16])?,
            mft_changed:            utils::read_bytes(&buf[16..24])?,
            file_read:              utils::read_bytes(&buf[24..32])?,
            file_permissions:       utils::read_bytes(&buf[32..36])?,
            max_num_of_versions:    utils::read_bytes(&buf[36..40])?,
            version_number:         utils::read_bytes(&buf[40..44])?,
            class_id:               utils::read_bytes(&buf[44..48])?,
            owner_id:               utils::read_bytes(&buf[48..52])?,
            security_id:            utils::read_bytes(&buf[52..56])?,
            quota_charged:          utils::read_bytes(&buf[56..64])?,
            update_sequence_number: utils::read_bytes(&buf[64..72])?,
        })
    }
    pub fn get(attribute: Attribute) -> Option<StandardInformation> {
        match attribute {
            Attribute::StandardInformation(si) => Some(si),
            _ => None,
        }
    }
}
