mod hash;

pub use hash::*;
pub use indexmap::IndexMap;
pub use std::fmt;
pub use std::str::FromStr;

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

pub fn get_asbr_adx2_file_index(filename: &str) -> u16 {
    match filename {
        "v_sys_etc"     => 1,
        "v_mob"         => 2,
        "v_btl_0bao01"  => 3,
        "v_btl_1dio01"  => 4,
        "v_btl_1jnt01"  => 5,
        "v_btl_1sdw01"  => 6,
        "v_btl_1zpl01"  => 7,
        "v_btl_2csr01"  => 8,
        "v_btl_2esd01"  => 9,
        "v_btl_2jsp01"  => 10,
        "v_btl_2krs01"  => 11,
        "v_btl_2lsa01"  => 12,
        "v_btl_2wmu01"  => 13,
        "v_btl_3abd01"  => 14,
        "v_btl_3dio01"  => 15,
        "v_btl_3hhs01"  => 16,
        "v_btl_3igy01"  => 17,
        "v_btl_3jsp01"  => 18,
        "v_btl_3jtr01"  => 19,
        "v_btl_3kki01"  => 20,
        "v_btl_3mra01"  => 21,
        "v_btl_3pln01"  => 22,
        "v_btl_3psp01"  => 23,
        "v_btl_3vni01"  => 24,
        "v_btl_4jsk01"  => 25,
        "v_btl_4jtr01"  => 26,
        "v_btl_4kir01"  => 27,
        "v_btl_4koi01"  => 28,
        "v_btl_4kwk01"  => 29,
        "v_btl_4oky01"  => 30,
        "v_btl_4oti01"  => 31,
        "v_btl_4rhn01"  => 32,
        "v_btl_4sgc01"  => 33,
        "v_btl_4ykk01"  => 34,
        "v_btl_5bct01"  => 35,
        "v_btl_5dvl01"  => 36,
        "v_btl_5fgo01"  => 37,
        "v_btl_5gac01"  => 38,
        "v_btl_5grn01"  => 39,
        "v_btl_5mst01"  => 40,
        "v_btl_5nrc01"  => 41,
        "v_btl_5prs01"  => 42,
        "v_btl_5trs01"  => 43,
        "v_btl_6ans01"  => 44,
        "v_btl_6elm01"  => 45,
        "v_btl_6fit01"  => 46,
        "v_btl_6jln01"  => 47,
        "v_btl_6pci02"  => 48,
        "v_btl_7dio01"  => 49,
        "v_btl_7jir01"  => 50,
        "v_btl_7jny01"  => 51,
        "v_btl_7vtn01"  => 52,
        "v_btl_8jsk01"  => 53,
        "v_card"        => 56,
        "v_gallery"     => 57,
        "v_btl_6wet01"  => 58,
        "v_sys_6wet01"  => 59,
        "v_btl_5ris01"  => 60,
        "v_sys_5ris01"  => 61,
        "v_btl_6pci01"  => 62,
        "v_sys_6pci01"  => 63,
        "v_btl_2shm01"  => 64,
        "v_sys_2shm01"  => 65,
        "v_btl_4kch01"  => 66,
        "v_sys_4kch01"  => 67,
        "v_btl_7dio02"  => 68,
        "v_sys_7dio02"  => 69,
        "v_btl_5abc01"  => 70,
        "v_sys_5abc01"  => 71,
        "v_btl_4fgm01"  => 72,
        "v_sys_4fgm01"  => 73,
        "v_btl_8wou01"  => 74,
        "v_sys_8wou01"  => 75,
        _ => 0, // [TODO] Support custom indices.
    }
}

pub fn get_asbr_adx2_filename(index: u16) -> String {
    match index {
        1  => "v_sys_etc".to_string(),
        2  => "v_mob".to_string(),
        3  => "v_btl_0bao01".to_string(),
        4  => "v_btl_1dio01".to_string(),
        5  => "v_btl_1jnt01".to_string(),
        6  => "v_btl_1sdw01".to_string(),
        7  => "v_btl_1zpl01".to_string(),
        8  => "v_btl_2csr01".to_string(),
        9  => "v_btl_2esd01".to_string(),
        10 => "v_btl_2jsp01".to_string(),
        11 => "v_btl_2krs01".to_string(),
        12 => "v_btl_2lsa01".to_string(),
        13 => "v_btl_2wmu01".to_string(),
        14 => "v_btl_3abd01".to_string(),
        15 => "v_btl_3dio01".to_string(),
        16 => "v_btl_3hhs01".to_string(),
        17 => "v_btl_3igy01".to_string(),
        18 => "v_btl_3jsp01".to_string(),
        19 => "v_btl_3jtr01".to_string(),
        20 => "v_btl_3kki01".to_string(),
        21 => "v_btl_3mra01".to_string(),
        22 => "v_btl_3pln01".to_string(),
        23 => "v_btl_3psp01".to_string(),
        24 => "v_btl_3vni01".to_string(),
        25 => "v_btl_4jsk01".to_string(),
        26 => "v_btl_4jtr01".to_string(),
        27 => "v_btl_4kir01".to_string(),
        28 => "v_btl_4koi01".to_string(),
        29 => "v_btl_4kwk01".to_string(),
        30 => "v_btl_4oky01".to_string(),
        31 => "v_btl_4oti01".to_string(),
        32 => "v_btl_4rhn01".to_string(),
        33 => "v_btl_4sgc01".to_string(),
        34 => "v_btl_4ykk01".to_string(),
        35 => "v_btl_5bct01".to_string(),
        36 => "v_btl_5dvl01".to_string(),
        37 => "v_btl_5fgo01".to_string(),
        38 => "v_btl_5gac01".to_string(),
        39 => "v_btl_5grn01".to_string(),
        40 => "v_btl_5mst01".to_string(),
        41 => "v_btl_5nrc01".to_string(),
        42 => "v_btl_5prs01".to_string(),
        43 => "v_btl_5trs01".to_string(),
        44 => "v_btl_6ans01".to_string(),
        45 => "v_btl_6elm01".to_string(),
        46 => "v_btl_6fit01".to_string(),
        47 => "v_btl_6jln01".to_string(),
        48 => "v_btl_6pci02".to_string(),
        49 => "v_btl_7dio01".to_string(),
        50 => "v_btl_7jir01".to_string(),
        51 => "v_btl_7jny01".to_string(),
        52 => "v_btl_7vtn01".to_string(),
        53 => "v_btl_8jsk01".to_string(),
        56 => "v_card".to_string(),
        57 => "v_gallery".to_string(),
        58 => "v_btl_6wet01".to_string(),
        59 => "v_sys_6wet01".to_string(),
        60 => "v_btl_5ris01".to_string(),
        61 => "v_sys_5ris01".to_string(),
        62 => "v_btl_6pci01".to_string(),
        63 => "v_sys_6pci01".to_string(),
        64 => "v_btl_2shm01".to_string(),
        65 => "v_sys_2shm01".to_string(),
        66 => "v_btl_4kch01".to_string(),
        67 => "v_sys_4kch01".to_string(),
        68 => "v_btl_7dio02".to_string(),
        69 => "v_sys_7dio02".to_string(),
        70 => "v_btl_5abc01".to_string(),
        71 => "v_sys_5abc01".to_string(),
        72 => "v_btl_4fgm01".to_string(),
        73 => "v_sys_4fgm01".to_string(),
        74 => "v_btl_8wou01".to_string(),
        75 => "v_sys_8wou01".to_string(),
        _  => index.to_string(),
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
