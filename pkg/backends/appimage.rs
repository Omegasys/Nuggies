use crate::pkg::{manager::PackageManager, package::{Package, PackageFormat}};

pub struct AppImageBackend;

impl AppImageBackend {
    pub fn new() -> Self {
        Self
    }
}

impl PackageManager for AppImageBackend {
    fn name(&self) -> &str {
        "appimage"
    }

    fn search(&self, _query: &str) -> Result<Vec<Package>, String> {
        Ok(vec![])
    }

    fn install(&self, _package: &str) -> Result<(), String> {
        Err("AppImage install is manual (download + execute)".into())
    }

    fn remove(&self, _package: &str) -> Result<(), String> {
        Err("AppImage removal is manual (delete file)".into())
    }

    fn update(&self, _package: Option<&str>) -> Result<(), String> {
        Err("AppImage updates depend on upstream".into())
    }

    fn info(&self, package: &str) -> Result<Package, String> {
        Ok(Package {
            name: package.to_string(),
            version: None,
            description: Some("AppImage binary (portable)".into()),
            format: PackageFormat::AppImage,
            dependencies: vec![],
            size: None,
            source: None,
            installed: false,
        })
    }
}
