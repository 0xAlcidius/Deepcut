#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

pub use crate::mft::attributes::file_name::FileName;
pub use crate::mft::attributes::standard_information::StandardInformation;

pub mod standard_information;
pub mod file_name;

#[derive(Clone)]
pub enum Attribute {
    StandardInformation(StandardInformation),
    FileName(FileName),
}