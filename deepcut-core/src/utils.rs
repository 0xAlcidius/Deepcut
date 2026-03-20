use crate::errors::CoreError;
use super::errors;
pub trait FromLeBytes: Sized {
    fn from_le_bytes(buf: &[u8]) -> Result<Self, CoreError>;
}

impl FromLeBytes for u8 {
    fn from_le_bytes(buf: &[u8]) -> Result<Self, CoreError> {
        buf.try_into().map(u8::from_le_bytes).map_err(|_| CoreError::CoreFailedToReadLeU8)
    }
}

impl FromLeBytes for u16 {
    fn from_le_bytes(buf: &[u8]) -> Result<Self, CoreError> {
        buf.try_into().map(u16::from_le_bytes).map_err(|_| CoreError::CoreFailedToReadLeU16)
    }
}

impl FromLeBytes for u32 {
    fn from_le_bytes(buf: &[u8]) -> Result<Self, CoreError> {
        buf.try_into().map(u32::from_le_bytes).map_err(|_| CoreError::CoreFailedToReadLeU32)
    }
}

impl FromLeBytes for u64 {
    fn from_le_bytes(buf: &[u8]) -> Result<Self, CoreError> {
        buf.try_into().map(u64::from_le_bytes).map_err(|_| CoreError::CoreFailedToReadLeU64)
    }
}

pub fn read_bytes<T: FromLeBytes>(buf: &[u8]) -> Result<T, CoreError> {
    T::from_le_bytes(buf)
}