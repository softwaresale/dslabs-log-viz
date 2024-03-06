use std::error;
use ratatui::widgets::ListState;
use dslabs_events::event::Event;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    /// the events to display
    pub events: Vec<Event>,
    /// list state used to control the main event list display
    pub event_list_state: ListState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            events: Default::default(),
            event_list_state: Default::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(events: Vec<Event>) -> Self {
        Self {
            events,
            ..Default::default()
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_event(&mut self) {
        let new_selected_idx = self.event_list_state.selected()
            .map(|curr| curr + 1)
            .unwrap_or(0);
        
        self.event_list_state.select(Some(new_selected_idx))
    }
    
    pub fn prev_event(&mut self) {
        let new_selected = self.event_list_state.selected()
            .filter(|selected| *selected > 0)
            .map(|sel| sel - 1);
        
        self.event_list_state.select(new_selected);
    }
    
    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
