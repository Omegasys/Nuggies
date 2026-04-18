use tui::{
    backend::Backend,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::App;

pub fn render<B: Backend>(f: &mut Frame<B>, _app: &App) {
    let block = Paragraph::new("Package details (not implemented)")
        .block(Block::default().title("Package").borders(Borders::ALL));

    f.render_widget(block, f.size());
}
