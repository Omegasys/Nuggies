use crate::gui::bridge::GuiBridge;

pub struct ActionButtons;

impl ActionButtons {
    pub fn render(package: &str) {
        println!("[Install] [Remove] [Rollback]");
        println!("Target: {}", package);
    }

    pub fn install(bridge: &mut GuiBridge, package: &str) {
        bridge.install_package(package);
    }

    pub fn install_github(bridge: &mut GuiBridge, owner: &str, repo: &str) {
        bridge.install_github(owner, repo);
    }
}
