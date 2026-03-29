use crate::errors::CoreError;
use super::{errors, utils};
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

pub fn extract_string(buf: &[u8], name_len: usize) -> Option<String> {
    if name_len == 0 {
        return None;
    }
    let byte_len = name_len * 2;
    if buf.len() < byte_len {
        return None;
    }
    let utf16: Vec<u16> = buf[..byte_len]
        .chunks_exact(2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .collect();
    String::from_utf16(&utf16).ok()
}