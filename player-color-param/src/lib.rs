use version::Version;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PlayerColorParam {
    pub version: Version,
    pub entries: HashMap<EntryKey, RGB>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct EntryKey {
    pub character_id: String,
    pub costume_index: u8,  // zero-indexed
    pub alt_index: u8,      // zero-indexed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
