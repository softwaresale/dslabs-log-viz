use crate::app::{App, AppResult, FocusedWindow};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
            Ok(())
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
            Ok(())
        }
        // Counter handlers
        KeyCode::Right => {
            // app.focused_window = FocusedWindow::FilterList;
            Ok(())
        }
        KeyCode::Left => {
            // app.focused_window = FocusedWindow::EventList;
            Ok(())
        }
        _ => {
            match &app.focused_window {
                FocusedWindow::EventList => {
                    events_list_handle_key_events(key_event, app)
                }
                FocusedWindow::FilterList => {
                    todo!()
                }
            }
        }
    }
}

fn events_list_handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // select up and down the event list
        KeyCode::Down => {
            app.event_list_state.next_event();
        }
        KeyCode::PageDown => {
            app.event_list_state.next_page();
        }
        KeyCode::Up => {
            app.event_list_state.prev_event();
        }
        KeyCode::PageUp => {
            app.event_list_state.prev_page();
        }
        KeyCode::Home => {
            app.event_list_state.go_home();
        }
        KeyCode::End => {
            app.event_list_state.go_end();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
