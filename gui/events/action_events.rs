#[derive(Debug, Clone)]
pub enum ActionEvent {
    InstallPackage(String),
    RemovePackage(String),
    UpdatePackage(String),
    RollbackPackage(String, String), // package, version
    InstallFromGitHub { owner: String, repo: String },
    RunAutomation(String),
}
