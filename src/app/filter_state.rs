use std::collections::HashSet;
use crate::ds_events::event::Event;
use crate::dsl::filters::{EventSequenceQuery};
use crate::dsl::query_ast::EventQuery;

#[derive(Default, Debug)]
pub struct FilterState {
    /// Our current event filter
    event_filter: Option<EventQuery>,
    /// the set of events that match this state
    matching_events: HashSet<usize>,
}

impl FilterState {
    
    pub fn matching_events(&self) -> &HashSet<usize> {
        &self.matching_events
    }
    
    pub fn clear_filter(&mut self) {
        self.matching_events.clear();
        self.event_filter = None;
    }
    
    pub fn push_new_filter(&mut self, event_query: EventQuery, events: &[Event]) {

        self.matching_events.clear();
        match &event_query {
            EventQuery::Find { queries } => {
                for query in queries {
                    match query.eval(events) {
                        Ok(matches) => {
                            self.matching_events.extend(matches.into_iter())
                        }
                        Err(_) => {
                            // TODO something this this
                        }
                    }
                }
            }
        }
        
        self.event_filter = Some(event_query);
    }
}
