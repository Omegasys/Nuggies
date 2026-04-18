use crate::gui::backend_bridge::{cli_bridge::CliBridge, runtime_bridge::RuntimeBridge};

pub struct PackageApi {
    pub runtime: RuntimeBridge,
}

impl PackageApi {
    pub fn new() -> Self {
        Self {
            runtime: RuntimeBridge::new(),
        }
    }

    // -------------------------
    // Core package actions
    // -------------------------

    pub fn install(&mut self, pkg: &str) {
        self.runtime.install_package(pkg);
    }

    pub fn remove(&self, pkg: &str) {
        let _ = CliBridge::run_command("echo", &["remove", pkg]);
    }

    pub fn update(&mut self) {
        self.runtime.update_all();
    }

    pub fn rollback(&self, pkg: &str, version: &str) {
        let _ = CliBridge::run_command("echo", &["rollback", pkg, version]);
    }

    // -------------------------
    // GitHub installs
    // -------------------------

    pub fn install_github(&mut self, owner: &str, repo: &str) {
        self.runtime.install_github(owner, repo);
    }

    // -------------------------
    // Lifecycle
    // -------------------------

    pub fn run(self) {
        self.runtime.run();
    }
}
