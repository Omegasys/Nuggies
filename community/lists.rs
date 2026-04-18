#[derive(Debug, Clone)]
pub struct PackageList {
    pub name: String,
    pub description: Option<String>,
    pub packages: Vec<String>,
    pub author: Option<String>,
}

impl PackageList {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            packages: vec![],
            author: None,
        }
    }

    pub fn add(&mut self, package: &str) {
        self.packages.push(package.to_string());
    }

    pub fn describe(&self) -> String {
        let mut out = format!("list: {}\n", self.name);

        if let Some(desc) = &self.description {
            out.push_str(&format!("description: {}\n", desc));
        }

        if let Some(author) = &self.author {
            out.push_str(&format!("author: {}\n", author));
        }

        out.push_str("packages:\n");

        for p in &self.packages {
            out.push_str(&format!("  - {}\n", p));
        }

        out
    }
}
