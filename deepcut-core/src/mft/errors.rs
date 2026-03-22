use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum MftError {
    #[error("Corrupted MFT Object")]
    MftHeaderSignatureBad,

    #[error("Invalid MFT Object")]
    MftHeaderSignatureInvalid,

    #[error("MFT Record Too Small")]
    MftRecordTooSmall,

    #[error("Failed to get Bytes from MFT Record")]
    MftRecordFailedToGetBytesFromRecord,

    #[error("Failed to read Standard Information, buffer too small")]
    MftAttributeStandardInformationSmallBuffer,



}