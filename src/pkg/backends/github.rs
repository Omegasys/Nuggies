use crate::runtime::executor::Executor;

pub struct GitHubBackend;

impl GitHubBackend {
    pub fn new() -> Self {
        Self
    }

    pub fn install(&self, owner: &str, repo: &str) {
        let mut exec = Executor::new(4);

        exec.github_install(owner, repo);

        exec.run();
    }
}
