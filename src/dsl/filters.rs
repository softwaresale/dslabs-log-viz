use crate::ds_events::event::Event;
use crate::dsl::query_ast::{Operator};

pub enum EventFilterError {
    KeyNotFound,
    MismatchTypes,
}

/// determines if we can accept an event
pub trait EventFilter {
    /// test an event. Returns true or false if it can be accepted, or an error if the query is bad
    fn test(&self, event: &Event) -> Result<bool, EventFilterError>;
}

impl<'query> EventFilter for Operator<'query> {
    fn test(&self, event: &Event) -> Result<bool, EventFilterError> {
        match self {
            Operator::Eq { prop_name, comparison } => {
                event.event_obj().props()
                    .get(*prop_name)
                    .ok_or(EventFilterError::KeyNotFound)
                    .and_then(|value| {
                        value.try_eq_raw_str(comparison)
                            .ok_or(EventFilterError::MismatchTypes)
                    })
            }
            Operator::Has(prop) => {
                Ok(event.event_obj().props().contains_key(*prop))
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
