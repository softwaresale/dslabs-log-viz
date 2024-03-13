use crate::app::{App, AppResult, FocusedWindow};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::dsl::parser::parse_event_query;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    
    if key_event.modifiers == KeyModifiers::CONTROL {
        match key_event.code {
            KeyCode::Right => {
                app.focused_window = FocusedWindow::QueryEditor;
                return Ok(())
            }
            KeyCode::Left => {
                app.focused_window = FocusedWindow::EventList;
                return Ok(())
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                app.quit();
                return Ok(())
            }
            _ => {}
        }
    }
    
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc => {
            app.quit();
            Ok(())
        }
        _ => {
            match &app.focused_window {
                FocusedWindow::EventList => {
                    events_list_handle_key_events(key_event, app)
                }
                FocusedWindow::QueryEditor => {
                    query_window_handle_key_events(key_event, app)
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
        KeyCode::Enter => {
            app.select_event(app.event_list_state.selected_event);
        }
        KeyCode::Char('c') => {
            app.clear_selected_event()
        }
        KeyCode::Char('n') => {
            if app.filter_state.has_active_filter() {
                // get the next one and focus it
                app.nav_next();
            } else {
                app.message_state.push("No query is active. Start a query to begin navigating")
            }
        }
        KeyCode::Char('N') => {
            if app.filter_state.has_active_filter() {
                // get the next one and focus it
                app.nav_prev();
            } else {
                app.message_state.push("No query is active. Start a query to begin navigating")
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

fn query_window_handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.modifiers == KeyModifiers::ALT {
        match key_event.code {
            KeyCode::Enter => {
                // TODO parse query
                let lines_buffer = app.query_text_area.lines().join("\n");
                match parse_event_query(&lines_buffer) {
                    Ok((_, event)) => {
                        app.push_new_filter_state(event);
                    }
                    Err(err) => {
                        let msg = format!("error while parsing query: {}", err);
                        app.message_state.push(msg);
                    }
                }
                return Ok(());
            },
            _ => {}
        }
    }
    
    if key_event.modifiers == KeyModifiers::CONTROL {
        match key_event.code {
            KeyCode::Backspace => {
                // TODO delete everything
            }
            _ => {}
        }
    }
    
    match key_event.code {
        _ => app.query_text_area.input(key_event),
    };
    
    Ok(())
}
