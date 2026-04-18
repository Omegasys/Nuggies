use crate::gui::state::GuiState;

pub struct LogsWindow;

impl LogsWindow {
    pub fn render(state: &GuiState) {
        println!("=== Logs Window ===");

        if state.logs.is_empty() {
            println!("No logs yet.");
            return;
        }

        for log in &state.logs {
            println!("{}", log);
        }
    }
}
