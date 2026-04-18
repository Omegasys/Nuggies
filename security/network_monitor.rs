#[derive(Debug, Clone)]
pub struct NetworkActivity {
    pub allowed: bool,
    pub endpoints: Vec<String>,
}

impl NetworkActivity {
    pub fn new() -> Self {
        Self {
            allowed: false,
            endpoints: vec![],
        }
    }

    pub fn describe(&self) -> String {
        if !self.allowed {
            return "network: blocked".into();
        }

        if self.endpoints.is_empty() {
            return "network: allowed (no endpoints listed)".into();
        }

        format!(
            "network: allowed -> {}",
            self.endpoints.join(", ")
        )
    }
}
