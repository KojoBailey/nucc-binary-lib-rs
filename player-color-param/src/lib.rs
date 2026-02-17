use std::fmt;

// [TODO] Make separate module for all structs to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Default for Version {
    fn default() -> Version {
        Version { major: 1, minor: 0, patch: 0 }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone)]
pub struct PlayerColorParam {
    pub version: Version,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub character_id: String,
    pub costume_index: u8,
    pub alt_index: u8,
    pub color: RGB,
}

impl Entry {
    pub fn key(&self) -> String {
        format!("{}_{}_{}", self.character_id, self.costume_index, self.alt_index)
    }
}
