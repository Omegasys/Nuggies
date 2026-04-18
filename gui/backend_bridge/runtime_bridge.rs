use crate::runtime::executor::Executor;

pub struct RuntimeBridge {
    executor: Executor,
}

impl RuntimeBridge {
    pub fn new() -> Self {
        Self {
            executor: Executor::new(4),
        }
    }

    pub fn install_package(&mut self, pkg: &str) {
        self.executor.install(pkg);
    }

    pub fn install_github(&mut self, owner: &str, repo: &str) {
        self.executor.github_install(owner, repo);
    }

    pub fn update_all(&mut self) {
        // future: push update job
    }

    pub fn run(self) {
        self.executor.run();
    }
}
