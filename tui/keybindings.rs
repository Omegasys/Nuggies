use crossterm::event::{KeyCode, KeyEvent};
use crate::tui::app::App;

pub fn handle_key(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Tab => app.next_view(),

        KeyCode::Char(c) => {
            app.search_query.push(c);
        }

        KeyCode::Backspace => {
            app.search_query.pop();
        }

        _ => {}
    }
}
