use tui::{
    backend::Backend,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::App;

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {
    let content = app.logs.join("\n");

    let block = Paragraph::new(content)
        .block(Block::default().title("Logs").borders(Borders::ALL));

    f.render_widget(block, f.size());
}
