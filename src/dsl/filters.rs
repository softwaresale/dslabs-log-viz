use std::collections::HashSet;
use crate::ds_events::event::Event;
use crate::dsl::query_ast::{EventNameFilter, FindEventNode, Operator};

pub enum EventFilterError {
    KeyNotFound,
    MismatchTypes,
}

/// determines if we can accept an event
pub trait EventFilter {
    /// test an event. Returns true or false if it can be accepted, or an error if the query is bad
    fn test(&self, event: &Event) -> Result<bool, EventFilterError>;
}

pub trait EventSequenceQuery {
    type ResT;

    fn eval(&self, events: &[Event]) -> Result<Self::ResT, EventFilterError>;
}

impl EventFilter for Operator {
    fn test(&self, event: &Event) -> Result<bool, EventFilterError> {
        match self {
            Operator::Eq { prop_name, comparison } => {
                event.event_obj().props()
                    .get(prop_name)
                    .ok_or(EventFilterError::KeyNotFound)
                    .and_then(|value| {
                        value.try_eq_raw_str(comparison)
                            .ok_or(EventFilterError::MismatchTypes)
                    })
            }
            Operator::Has(prop) => {
                Ok(event.event_obj().props().contains_key(prop))
            }
            Operator::Not(op) => {
                op.test(event)
                    .map(|val| !val)
            }
            Operator::And(ops) => {
                for op in ops {
                    if !op.test(event)? {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            Operator::Or(ops) => {
                for op in ops {
                    if op.test(event)? {
                        return Ok(true)
                    }
                }

                Ok(false)
            }
        }
    }
}

impl EventFilter for EventNameFilter {
    fn test(&self, event: &Event) -> Result<bool, EventFilterError> {
        match self {
            EventNameFilter::Any => Ok(true),
            EventNameFilter::Named(event_name) => Ok(event.event_obj().name() == event_name)
        }
    }
}

impl EventSequenceQuery for FindEventNode {
    type ResT = HashSet<usize>;

    fn eval(&self, events: &[Event]) -> Result<Self::ResT, EventFilterError> {
        let mut matches = HashSet::<usize>::new();
        for event in events {
            match self.event_type.test(event) {
                Ok(passes) => {
                    if !passes {
                        continue;
                    }
                }
                Err(_) => {
                    // TODO use this error somehow
                    continue;
                }
            }
            
            match self.operator.test(event) {
                Ok(passes) => {
                    if passes {
                        matches.insert(event.id());
                    }
                }
                Err(_) => {
                    // TODO use this somehow
                }
            }
        }
        
        Ok(matches)
    }
}
