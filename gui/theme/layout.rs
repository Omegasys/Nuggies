#[derive(Debug, Clone)]
pub struct Layout {
    pub padding: u8,
    pub margin: u8,
    pub gap: u8,
    pub border_radius: u8,
}

impl Layout {
    pub fn default() -> Self {
        Self {
            padding: 8,
            margin: 8,
            gap: 6,
            border_radius: 6,
        }
    }
}
