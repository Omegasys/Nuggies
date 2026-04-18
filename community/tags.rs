#[derive(Debug, Clone)]
pub struct Tag {
    pub package: String,
    pub tags: Vec<String>,
}

impl Tag {
    pub fn new(package: &str, tags: Vec<&str>) -> Self {
        Self {
            package: package.to_string(),
            tags: tags.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "{} tags: {}",
            self.package,
            self.tags.join(", ")
        )
    }
}
