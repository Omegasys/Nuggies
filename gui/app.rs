use crate::gui::state::GuiState;
use crate::gui::bridge::GuiBridge;

pub struct App {
    pub state: GuiState,
    pub bridge: GuiBridge,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: GuiState::new(),
            bridge: GuiBridge::new(),
        }
    }

    pub fn search(&mut self, query: &str) {
        self.state.search_query = query.to_string();
        self.state.push_log(&format!("Searching: {}", query));
    }

    pub fn install_selected(&mut self) {
        if let Some(pkg) = &self.state.selected_package {
            self.state.push_log(&format!("Installing {}", pkg));
            self.bridge.install_package(pkg);
        }
    }

    pub fn install_github(&mut self, owner: &str, repo: &str) {
        self.state.push_log(&format!("Installing GitHub {}/{}", owner, repo));
        self.bridge.install_github(owner, repo);
    }

    pub fn tick(&mut self) {
        // placeholder for UI refresh loop
        self.state.status_message = "Running".into();
    }
}
