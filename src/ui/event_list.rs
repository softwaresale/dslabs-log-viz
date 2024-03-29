use std::cmp::min;
use std::collections::{BTreeSet};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Color;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget};
use crate::ds_events::event::Event;

pub struct EventList<'events> {
    events: &'events [Event],
    selected: bool,
    matching_events: &'events BTreeSet<usize>,
}

impl<'events> EventList<'events> {
    pub fn new(events: &'events [Event], matching_events: &'events BTreeSet<usize>, selected: bool) -> Self {
        Self {
            events,
            selected,
            matching_events
        }
    }
}

#[derive(Default, Debug)]
pub struct EventListState {
    /// NOTE this does not change
    event_count: usize,
    /// the last time we checked, how many lines where there
    last_height: usize,
    /// how many pages total we have based on line height, last time we checked
    page_count: usize,
    /// which page is currently selected
    current_page: usize,
    /// which event is selected overall
    pub(crate) selected_event: usize,
    /// state for the given page
    page_state: ListState,
}

impl EventListState {

    pub fn new(event_count: usize, selected: Option<usize>) -> Self {
        Self {
            event_count,
            page_state: ListState::default().with_selected(selected),
            ..Self::default()
        }
    }

    pub fn go_home(&mut self) {
        self.selected_event = 0;
        self.page_state = ListState::default().with_selected(Some(0));
        self.current_page = 0;
    }
    
    pub fn next_event(&mut self) {
        // do nothing if we are at the end of the event count
        if self.selected_event >= self.event_count {
            return;
        }

        // bump the selected event
        self.selected_event += 1;

        let mut page_selected_value = self.page_state.selected().unwrap();
        page_selected_value += 1;
        // if we have gone beyond, we need to bump pages
        if page_selected_value >= self.last_height {
            self.next_page();
        } else {
            // otherwise, we can just update the page state
            *self.page_state.selected_mut() = Some(page_selected_value);
        }
    }
    
    pub fn prev_event(&mut self) {
        if self.selected_event == 0 {
            return;
        }

        // bump the selected event
        self.selected_event -= 1;

        let page_selected_value = self.page_state.selected().unwrap();
        if page_selected_value == 0 {
            self.prev_page()
        } else {
            *self.page_state.selected_mut() = Some(page_selected_value - 1);
        }
    }
    
    pub fn next_page(&mut self) {
        // do nothing if we are at the end
        if self.current_page >= self.page_count {
            return;
        }

        // update selected event
        let selected_page_idx = self.page_state.selected().unwrap_or(0);
        let dist_from_bottom = self.last_height - selected_page_idx;
        self.selected_event += dist_from_bottom;
        
        // update selected page state
        self.current_page += 1;
        self.page_state = ListState::default().with_selected(Some(0));
    }
    
    pub fn prev_page(&mut self) {
        if self.current_page == 0 {
            self.selected_event = 0;
            *self.page_state.selected_mut() = Some(0);
            return;
        }

        let last_selected = self.page_state.selected().unwrap_or(0);
        self.selected_event -= last_selected;
        self.selected_event -= 1;
        
        self.current_page -= 1;
        self.page_state = ListState::default().with_selected(Some(self.last_height - 1))

        // TODO update selected accordingly
    }
    
    pub fn go_end(&mut self) {
        self.current_page = self.page_count;
        self.selected_event = self.event_count - 1;
        
        // compute the last item
        let remaining_items = self.event_count - (self.page_count * self.last_height);
        
        self.page_state = ListState::default().with_selected(Some(min(remaining_items - 1, self.last_height - 1)));
    }
    
    fn calculate_pages(&mut self, height: usize) {
        self.last_height = height;
        self.page_count = self.event_count / height;
        if self.page_state.selected().is_some_and(|sel| sel > height) {
            *self.page_state.selected_mut() = Some(height - 1);
        }
    }
    
    pub fn focus_event(&mut self, event_idx: usize) {
        // too far
        if event_idx >= self.event_count {
            return;
        }

        // figure out which page we need to focus
        let Some((containing_page, page_offset)) = self.compute_containing_page(event_idx) else {
            panic!()
        };
        
        // set the selected page and event
        self.current_page = containing_page;
        self.selected_event = event_idx;
        *self.page_state.selected_mut() = Some(page_offset);
    }
    
    fn compute_containing_page(&self, event_idx: usize) -> Option<(usize, usize)> {
        let containing_page = event_idx / self.last_height;
        assert!(containing_page < self.page_count);
        
        let page_offset = event_idx - (containing_page * self.last_height);
        
        Some((containing_page, page_offset))
    }
}

impl<'events> StatefulWidget for EventList<'events> {
    type State = EventListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        
        let block = Block::bordered()
            .title("Events")
            .border_style(if self.selected {
                Style::default().fg(Color::Blue)
            } else {
                Style::default()
            });
        
        let block_area = block.inner(area);
        
        let window_height = block_area.height as usize;
        
        // re-calculate each time based on how many lines we can show
        state.calculate_pages(window_height);

        // figure out which chunk to display
        let events_iter = self.events
            .chunks(window_height)
            .nth(state.current_page)
            .into_iter()
            .flat_map(|event| {
                event.into_iter()
                    .map(|event| event_to_list_item(event, self.matching_events))
            });
        
        block.render(area, buf);
        
        StatefulWidget::render(
            List::new(events_iter)
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true),
            block_area,
            buf,
            &mut state.page_state
        );
    }
}

fn event_to_list_item<'ev>(event: &'ev Event, matching_events: &'ev BTreeSet<usize>) -> ListItem<'ev> {
    let mut item = ListItem::new(format!("{} {}: {}", event.id(), event.originator(), event.event_obj()));
    if matching_events.contains(&event.id()) {
        item = item.style(Style::default().fg(Color::Yellow).underlined());
    }
    
    item
}
