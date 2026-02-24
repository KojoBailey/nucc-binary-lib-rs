mod hash;

pub use hash::*;
pub use indexmap::IndexMap;

use std::fmt;
use std::str::FromStr;

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

#[derive(Debug, Clone, Eq, PartialEq)]
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

impl FromStr for Language {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "English"            => Ok(Language::English),
            "Spanish"            => Ok(Language::Spanish),
            "German"             => Ok(Language::German),
            "Italian"            => Ok(Language::Italian),
            "French"             => Ok(Language::French),
            "Japanese"           => Ok(Language::Japanese),
            "Korean"             => Ok(Language::Korean),
            "ChineseSimplified"  => Ok(Language::ChineseSimplified),
            "ChineseTraditional" => Ok(Language::ChineseTraditional),
            other                => Ok(Language::Other(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_to_string() {
        assert_eq!(Language::English.to_string(), "English");
        assert_eq!(Language::Other("Fante".to_string()).to_string(), "Fante");
    }

    #[test]
    fn string_to_language() {
        assert_eq!(Language::from_str("English").unwrap(), Language::English);
        assert_eq!(Language::from_str("Irish").unwrap(), Language::Other("Irish".to_string()));
    }
}
