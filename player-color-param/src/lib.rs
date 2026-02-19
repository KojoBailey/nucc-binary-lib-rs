pub use indexmap::IndexMap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct PlayerColorParam {
    pub entries: IndexMap<EntryKey, RGB>,
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

impl Ord for EntryKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.character_id.cmp(&other.character_id)
            .then(self.costume_index.cmp(&other.costume_index))
            .then(self.alt_index.cmp(&other.alt_index))
    }
}

impl PartialOrd for EntryKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl RGB {
    // Either "RRGGBB" or "#RRGGBB"
    pub fn from_hex_str(s: &str) -> Option<Self> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 {
            return None;
        }

        let red = u8::from_str_radix(&s[0..2], 16).ok()?;
        let green = u8::from_str_radix(&s[2..4], 16).ok()?;
        let blue = u8::from_str_radix(&s[4..6], 16).ok()?;

        Some(RGB { red, green, blue })
    }

    pub fn to_hex_str(&self, prepend_hashtag: bool) -> String {
        format!("{}{:02X}{:02X}{:02X}",
            if prepend_hashtag { "#" } else { "" },
            self.red, self.green, self.blue
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex_str() {
        let col = RGB::from_hex_str("0099FF").unwrap();
        assert_eq!(col, RGB { red: 0x00, green: 0x99, blue: 0xFF });
    }

    #[test]
    fn from_hex_str_with_hashtag() {
        let col = RGB::from_hex_str("#FF0099").unwrap();
        assert_eq!(col, RGB { red: 0xFF, green: 0x00, blue: 0x99 });
    }

    #[test]
    fn to_hex_str() {
        let rgb = RGB { red: 0x00, green: 0x99, blue: 0xFF };
        assert_eq!(rgb.to_hex_str(false), "0099FF");
    }

    #[test]
    fn to_hex_str_with_hashtag() {
        let rgb = RGB { red: 0xFF, green: 0x00, blue: 0x99 };
        assert_eq!(rgb.to_hex_str(true), "#FF0099");
    }
}
