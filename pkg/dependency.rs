use crate::pkg::version::Version;

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub required_version: Option<Version>,
    pub optional: bool,
}

impl Dependency {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            required_version: None,
            optional: false,
        }
    }
}
