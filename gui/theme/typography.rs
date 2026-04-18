#[derive(Debug, Clone)]
pub struct Typography {
    pub font_family: String,
    pub base_size: u8,
    pub heading_size: u8,
}

impl Typography {
    pub fn default() -> Self {
        Self {
            font_family: "monospace".into(),
            base_size: 14,
            heading_size: 18,
        }
    }
}
