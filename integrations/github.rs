use crate::pkg::package::Package;

#[derive(Debug, Clone)]
pub struct GitHubRepo {
    pub owner: String,
    pub repo: String,
}

impl GitHubRepo {
    pub fn new(owner: &str, repo: &str) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    pub fn url(&self) -> String {
        format!("https://github.com/{}/{}", self.owner, self.repo)
    }

    pub fn fetch_releases(&self) -> Result<Vec<Package>, String> {
        // Placeholder: no API dependency yet
        Ok(vec![])
    }
}
