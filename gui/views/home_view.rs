use crate::gui::state::GuiState;

pub struct HomeView;

impl HomeView {
    pub fn render(state: &GuiState) {
        println!("🏠 Nuggies Home");

        println!("Status: {}", state.status_message);

        println!("\nQuick Stats:");
        println!("- Search query: {}", state.search_query);
        println!("- Logs: {} entries", state.logs.len());

        println!("\nSuggested actions:");
        println!("- Search packages");
        println!("- View updates");
        println!("- Install from GitHub");
    }
}
