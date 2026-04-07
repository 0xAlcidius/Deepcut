use std::collections::HashMap;
use crate::mft_parser::MftEntry;

struct Node {
    pub record_number: u32,
    pub name: String,
    pub parent: u32,
    pub children: Vec<u32>,
}

pub fn build_tree(raw_mft: &HashMap<u64, MftEntry>) -> HashMap<u32, Node>{
    let mut nodes: HashMap<u32, Node> = HashMap::new();
    for entry in raw_mft.values() {
        /* iterate over $FILE_NAME attributes ->
        Get the FILE_NAME attribute on position 1 (WIN32) and position 3 (WIN32&DOS) ->
        If none of them exist get the attribute at position 0 (POSIX) ->
        If this vector is empty, simply make the name "unknown".
        */
        let name = entry.file_names.iter()
            .find(|f| f.filename_space == 1 || f.filename_space == 3)
            .or_else(|| entry.file_names.first())
            .and_then(|f| f.name.as_deref())
            .unwrap_or("unknown");

        let mut children: Vec<u32> = Vec::new();

        raw_mft.values().for_each(|e | {
            let parent = get_ref_to_parent(e);
            if entry.header.record_number == parent {
                children.push(e.header.record_number);
            }
        });

        nodes.insert(entry.header.record_number, Node {
            record_number: entry.header.record_number,
            name: String::from(name),
            parent: get_ref_to_parent(entry),
            children,
        });
    }
    nodes
}

fn get_ref_to_parent(entry: &MftEntry) -> u32 {
    let ref_to_parent = entry.file_names.iter()
        .find(|f| f.filename_space == 1 || f.filename_space == 3)
        .or_else(|| entry.file_names.first())
        .map(|f| f.ref_to_parent)
        .unwrap_or(5);

    (ref_to_parent & 0x0000_FFFF_FFFF_FFFF) as u32
}