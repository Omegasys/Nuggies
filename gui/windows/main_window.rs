use crate::gui::state::GuiState;
use crate::gui::bridge::GuiBridge;

pub struct MainWindow;

impl MainWindow {
    pub fn render(state: &mut GuiState, _bridge: &mut GuiBridge) {
        println!("=== Nuggies Main Window ===");

        println!("Search: {}", state.search_query);
        println!("Status: {}", state.status_message);

        println!("--- Logs Preview ---");
        for log in state.logs.iter().take(5) {
            println!("  {}", log);
        }
    }
}
