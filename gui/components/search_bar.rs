use crate::gui::state::GuiState;

pub struct SearchBar;

impl SearchBar {
    pub fn render(state: &mut GuiState) {
        println!("🔎 Search: [{}]", state.search_query);
        println!("(type to update query)");
    }

    pub fn update_query(state: &mut GuiState, query: &str) {
        state.search_query = query.to_string();
        state.push_log(&format!("Search updated: {}", query));
    }
}
