pub struct PlayerColorParam {
    character_id: String,
    costume_index: u8,
    alt_index: u8,
    red: u8,
    green: u8,
    blue: u8,
}

impl PlayerColorParam {
    pub fn key(&self) -> String {
        format!("{}_{}_{}", self.character_id, self.costume_index, self.alt_index)
    }
}
