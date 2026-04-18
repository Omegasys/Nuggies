use crate::gui::state::GuiState;

pub struct SettingsWindow;

impl SettingsWindow {
    pub fn render(state: &mut GuiState) {
        println!("=== Settings ===");

        println!("Current status: {}", state.status_message);

        println!("(Settings would include:)");
        println!("- Default package backend");
        println!("- Parallel download limit");
        println!("- Security level");
        println!("- Transparency verbosity");
    }
}
