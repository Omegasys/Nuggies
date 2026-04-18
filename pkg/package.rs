use crate::pkg::{dependency::Dependency, version::Version};

#[derive(Debug, Clone)]
pub enum PackageFormat {
    Native,
    Flatpak,
    Snap,
    AppImage,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: Option<Version>,
    pub description: Option<String>,
    pub format: PackageFormat,

    pub dependencies: Vec<Dependency>,

    // transparency fields
    pub size: Option<u64>,
    pub source: Option<String>,
    pub installed: bool,
}

impl Package {
    pub fn new(name: &str, format: PackageFormat) -> Self {
        Self {
            name: name.to_string(),
            version: None,
            description: None,
            format,
            dependencies: Vec::new(),
            size: None,
            source: None,
            installed: false,
        }
    }
}
