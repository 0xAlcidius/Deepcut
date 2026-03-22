pub struct FileName {
    pub ref_to_parent: u64,
    pub file_creation: u64,
    pub file_altered: u64,
    pub file_changed: u64,
    pub file_read: u64,
    pub allocated_size: u64,
    pub real_size: u64,
    pub flags: u32,
    pub eas_and_reparse: u32,
    pub filename_len_in_chars: u8,
    pub filename_space: u8,
    pub name: Option<String>,
}