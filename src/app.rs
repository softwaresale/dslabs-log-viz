mod filter_state;
mod messages_state;

use std::error;
use tui_textarea::TextArea;
use crate::app::filter_state::FilterState;
use crate::app::messages_state::MessagesState;
use crate::ds_events::event::Event;
use crate::ui::event_list::EventListState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum FocusedWindow {
    EventList,
    QueryEditor,
}

impl FocusedWindow {
    pub fn is_event_list(&self) -> bool {
        match self {
            FocusedWindow::EventList => true,
            _ => false
        }
    }

    pub fn is_filter_list(&self) -> bool {
        match self {
            FocusedWindow::QueryEditor => true,
            _ => false,
        }
    }
}

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// the current portion of the screen we have focused
    pub focused_window: FocusedWindow,
    /// the events to display
    pub events: Vec<Event>,
    /// list state used to control the main event list display
    pub event_list_state: EventListState,
    /// our query text editor + state
    pub query_text_area: TextArea<'a>,
    /// used for filtering the events
    pub filter_state: FilterState,
    pub message_state: MessagesState,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            running: true,
            focused_window: FocusedWindow::EventList,
            events: Default::default(),
            event_list_state: Default::default(),
            query_text_area: Default::default(),
            filter_state: Default::default(),
            message_state: Default::default(),
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new(events: Vec<Event>) -> Self {
        
        let starting_index = if !events.is_empty() {
            Some(0usize)
        } else {
            None
        };
        
        let event_count = events.len();
        
        Self {
            events,
            event_list_state: EventListState::new(event_count, starting_index),
            ..Default::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
