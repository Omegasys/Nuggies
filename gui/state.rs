#[derive(Debug, Clone)]
pub struct GuiState {
    pub search_query: String,
    pub selected_package: Option<String>,
    pub logs: Vec<String>,
    pub is_loading: bool,
    pub status_message: String,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            selected_package: None,
            logs: vec![],
            is_loading: false,
            status_message: "Ready".into(),
        }
    }

    pub fn push_log(&mut self, msg: &str) {
        self.logs.push(msg.to_string());
    }
}
