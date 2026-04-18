#[derive(Debug, Clone)]
pub enum JobType {
    Install(String),
    Remove(String),
    Update,
    GitHubInstall(String, String), // owner, repo
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: u64,
    pub job_type: JobType,
}
