use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('r') | KeyCode::Char('R') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.context.system.refresh_all();
            }
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Tab => {
            // Hardcoded number of tabs for now
            // TODO: Make this dynamic

            let hi = 2;
            if app.context.tab_index == hi {
                app.context.tab_index = 0;
            } else {
                app.context.tab_index += 1;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.context.row_index > 0 {
                app.context.row_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.context.row_index < app.context.system.processes().len() - 1 {
                app.context.row_index += 1;
            }
        }
        // KeyCode::Right | KeyCode::Char('j') => app.increment_counter(),
        // KeyCode::Left | KeyCode::Char('k') => app.decrement_counter(),
        _ => {}
    };
}
