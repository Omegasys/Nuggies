use crate::tui::views::{logs_view, package_view, search_view};

pub enum View {
    Search,
    Package,
    Logs,
}

pub struct App {
    pub current_view: View,
    pub running: bool,

    // shared state (expand later)
    pub search_query: String,
    pub logs: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_view: View::Search,
            running: true,
            search_query: String::new(),
            logs: vec!["Nuggies started".into()],
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_view(&mut self) {
        self.current_view = match self.current_view {
            View::Search => View::Package,
            View::Package => View::Logs,
            View::Logs => View::Search,
        };
    }

    pub fn render<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
    ) {
        match self.current_view {
            View::Search => search_view::render(f, self),
            View::Package => package_view::render(f, self),
            View::Logs => logs_view::render(f, self),
        }
    }
}
