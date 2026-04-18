#[derive(Debug, Clone)]
pub struct RemoteRepo {
    pub url: String,
}

impl RemoteRepo {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub fn fetch_index(&self) -> Result<String, String> {
        // Placeholder: later could be HTTP / OCI / git / custom protocol
        Ok(format!("fetching index from {}", self.url))
    }

    pub fn list_packages(&self) -> Result<Vec<String>, String> {
        // intentionally undefined at this stage
        Ok(vec![])
    }
}
