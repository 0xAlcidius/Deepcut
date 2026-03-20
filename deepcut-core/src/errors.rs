use crate::mft::errors::MftError;
use thiserror::Error;


#[derive(PartialEq, Debug)]
pub enum ERROR {
    Core(CoreError),
    Mft(MftError),
}

impl From<MftError> for ERROR {
    fn from(e: MftError) -> Self {
        ERROR::Mft(e)
    }
}

impl From<CoreError> for ERROR {
    fn from(e: CoreError) -> Self {
        ERROR::Core(e)
    }
}

#[derive(Error, PartialEq, Debug)]
pub enum CoreError {
    #[error("Failed to read LE U8 bytes")]
    CoreFailedToReadLeU8,

    #[error("Failed to read LE U16 bytes")]
    CoreFailedToReadLeU16,

    #[error("Failed to read LE U32 bytes")]
    CoreFailedToReadLeU32,

    #[error("Failed to read LE U64 bytes")]
    CoreFailedToReadLeU64,
}