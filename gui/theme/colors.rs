#[derive(Debug, Clone)]
pub struct Colors {
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub warning: String,
    pub error: String,
    pub success: String,
}

impl Colors {
    pub fn dark() -> Self {
        Self {
            background: "#1e1e1e".into(),
            foreground: "#eaeaea".into(),
            accent: "#4caf50".into(),
            warning: "#ff9800".into(),
            error: "#f44336".into(),
            success: "#8bc34a".into(),
        }
    }
}
