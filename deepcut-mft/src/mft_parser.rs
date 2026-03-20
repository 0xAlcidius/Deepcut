use std::fs::File;
use std::io::Read;
use deepcut_core::mft::structure::{MftHeader, MFT_RECORD_SIZE};
use deepcut_core::errors;
use deepcut_core::errors::ERROR;
use deepcut_core::mft::errors::MftError;
use crate::vprintln;

pub fn parse(path: &str) {
    let mut contents = match read_file(path) {
        Ok(contents) => contents,
        Err(e) => {
            vprintln!("Could not read file {}: {}", path, e);
            return;
        }
    };

    let mut good_records = 0;
    let mut corrupted_records = 0;
    let mut invalid_records = 0;

    let mut pointer = 0;
    loop {
        if pointer + MFT_RECORD_SIZE > contents.len() {
            break;
        }

        let record = &mut contents[pointer..pointer+MFT_RECORD_SIZE];
        pointer += MFT_RECORD_SIZE;

        let mft_header = match MftHeader::parse(record) {
            Ok(header) => header,
            Err(e) => {
                if e == ERROR::from(MftError::MftHeaderSignatureBad) {
                    corrupted_records += 1;
                } else if e == ERROR::from(MftError::MftHeaderSignatureInvalid) {
                    invalid_records += 1;
                }
                continue;
            }
        };
        good_records += 1;
    }
    vprintln!("Good record count: {}\nCorrupted records: {}\nInvalid records: {}", good_records, corrupted_records, invalid_records);
}

fn read_file(path: &str) -> Result<Vec<u8>, &'static str>{
    vprintln!("Parsing {}", path);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            vprintln!("Could not open file: {}", e);
            return Err("Could not open file");
        }
    };

    let mut contents = Vec::new();
    return match file.read_to_end(&mut contents) {
        Ok(_) => Ok(contents),
        Err(_) => Err("Failed to read file"),
    };
}