mod hash;

pub use hash::*;
pub use indexmap::IndexMap;

use std::fmt;

#[derive(Debug, Clone)]
pub struct MessageInfo {
    pub language: Language,
    pub entries: IndexMap<u32, Entry>,
}

impl MessageInfo {
    // The "other" takes priority.
    pub fn merge(&mut self, other: &Self) {
        self.language = other.language.clone();
        self.entries.extend(other.entries.iter().map(|(k, v)| (k.clone(), v.clone())));
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Entry {
    pub string_id: Option<String>,
    pub message: Option<String>,
    pub reference_id: Option<Reference>,
    pub adx2_file: Option<String>,
    pub adx2_cue_index: Option<u16>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Reference {
    StringId(String),
    HashId(u32),
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
    ChineseTraditional,
    Other(String), // ISO 639 Set 3
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::English            => write!(f, "English"),
            Language::Spanish            => write!(f, "Spanish"),
            Language::German             => write!(f, "German"),
            Language::Italian            => write!(f, "Italian"),
            Language::French             => write!(f, "French"),
            Language::Japanese           => write!(f, "Japanese"),
            Language::Korean             => write!(f, "Korean"),
            Language::ChineseSimplified  => write!(f, "ChineseSimplified"),
            Language::ChineseTraditional => write!(f, "ChineseTraditional"),
            Language::Other(custom)      => write!(f, "{}", custom),
        }
    }
}
