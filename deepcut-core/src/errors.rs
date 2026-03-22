use crate::mft::errors::MftError;
use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum DeepcutError {
    #[error(transparent)]
    Core(CoreError),

    #[error(transparent)]
    Mft(MftError),
}

impl From<MftError> for DeepcutError {
    fn from(e: MftError) -> Self {
        DeepcutError::Mft(e)
    }
}

impl From<CoreError> for DeepcutError {
    fn from(e: CoreError) -> Self {
        DeepcutError::Core(e)
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

    #[error("Failed to parse UTF16")]
    CoreFailedToParseUtf16,
}