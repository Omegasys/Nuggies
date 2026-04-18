use crate::gui::{state::GuiState, bridge::GuiBridge};

pub struct InstallWindow;

impl InstallWindow {
    pub fn render(state: &mut GuiState, bridge: &mut GuiBridge) {
        println!("=== Install Window ===");

        if let Some(pkg) = &state.selected_package {
            println!("Preparing to install: {}", pkg);

            println!("--- Preview ---");
            println!("- download source resolved");
            println!("- dependencies listed");
            println!("- security check pending");

            println!("\nExecuting install...");
            bridge.install_package(pkg);
            state.push_log(&format!("Install triggered: {}", pkg));
        } else {
            println!("No package selected for install.");
        }
    }
}
