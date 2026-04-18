#[derive(Debug, Clone)]
pub struct Warning {
    pub package: String,
    pub messages: Vec<String>,
}

impl Warning {
    pub fn new(package: &str) -> Self {
        Self {
            package: package.to_string(),
            messages: vec![],
        }
    }

    pub fn add_warning(&mut self, msg: &str) {
        self.messages.push(msg.to_string());
    }

    pub fn describe(&self) -> String {
        if self.messages.is_empty() {
            return format!("{}: no warnings", self.package);
        }

        let mut out = format!("{} warnings:\n", self.package);

        for w in &self.messages {
            out.push_str(&format!("  - {}\n", w));
        }

        out
    }
}
