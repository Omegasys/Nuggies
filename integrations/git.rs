use crate::core::system;

#[derive(Debug, Clone)]
pub struct GitRepo {
    pub url: String,
    pub branch: Option<String>,
}

impl GitRepo {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            branch: None,
        }
    }

    pub fn clone(&self, path: &str) -> Result<(), String> {
        let mut args = vec!["clone", &self.url, path];

        system::run_command("git", &args)?;
        Ok(())
    }

    pub fn pull(&self, path: &str) -> Result<(), String> {
        system::run_command("git", &["-C", path, "pull"])?;
        Ok(())
    }
}
