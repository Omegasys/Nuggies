#[derive(Debug, Clone)]
pub struct PackageCard {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
}

impl PackageCard {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: None,
            description: None,
        }
    }

    pub fn render(&self) {
        println!("📦 {}", self.name);

        if let Some(v) = &self.version {
            println!("   version: {}", v);
        }

        if let Some(d) = &self.description {
            println!("   {}", d);
        }
    }
}
