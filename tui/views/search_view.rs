use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::App;

pub fn render<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.size());

    let input = Paragraph::new(app.search_query.as_ref())
        .block(Block::default().title("Search").borders(Borders::ALL));

    let results = Paragraph::new("Results will appear here")
        .block(Block::default().title("Results").borders(Borders::ALL));

    f.render_widget(input, chunks[0]);
    f.render_widget(results, chunks[1]);
}
