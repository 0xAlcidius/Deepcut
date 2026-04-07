use std::collections::HashMap;
use crate::mft_parser::MftEntry;

struct Node {
    pub record_number: u32,
    pub name: String,
    pub parent: u32,
    pub children: Vec<u32>,
    pub is_dir: bool,
    pub entry: MftEntry,
}

pub fn build_tree(raw_mft: &HashMap<u64, MftEntry>) {
    for entry in raw_mft.values() {
        let name = entry.file_names.iter()
            .find(|f| f.filename_space == 1 || f.filename_space == 3)
            .or_else(|| entry.file_names.first())
            .and_then(|f| f.name.as_deref())
            .unwrap_or("[unknown]");
        println!("{}", name);
    }
}