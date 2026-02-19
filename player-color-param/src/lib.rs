use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PlayerColorParam {
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
