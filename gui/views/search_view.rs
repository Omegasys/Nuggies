use crate::gui::state::GuiState;

pub struct SearchView;

impl SearchView {
    pub fn render(state: &GuiState) {
        println!("🔎 Search View");

        println!("Query: {}", state.search_query);

        println!("\nResults (placeholder):");
        println!("  - firefox");
        println!("  - vim");
        println!("  - neovim");
        println!("  - rustup");
    }
}
