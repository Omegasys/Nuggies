use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct LocalRepo {
    pub path: String,
}

impl LocalRepo {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    pub fn list_packages(&self) -> Result<Vec<String>, String> {
        let entries = fs::read_dir(&self.path)
            .map_err(|e| e.to_string())?;

        let mut packages = vec![];

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            if let Some(name) = entry.file_name().to_str() {
                packages.push(name.to_string());
            }
        }

        Ok(packages)
    }
}
