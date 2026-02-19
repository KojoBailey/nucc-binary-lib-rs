pub use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct MessageInfo {
    pub language: Language,
    pub entries: IndexMap<u64, Entry>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Entry {
    pub hash_id: u64,
    pub string_id: Option<String>,
    pub message: Option<String>,
    pub reference_id: Option<String>,
    pub adx2_file: Option<String>,
    pub adx2_cue_index: Option<u64>,
}

impl Entry {
    pub fn key(&self) -> u64 { self.hash_id }
}

#[derive(Debug, Clone)]
pub enum Language {
    // All officially supported languages in one game or another.
    English,
    Spanish,
    German,
    Italian,
    French,
    Japanese,
    Korean,
    ChineseSimplified,
    ChineseTraditionnal,
    Other(String), // ISO 639 Set 3
}
