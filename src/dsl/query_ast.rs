use crate::ds_events::abstract_object::AbstractValue;
use crate::ds_events::event::Event;
use crate::dsl::filters::EventFilterError;

/// top level query object
#[derive(Debug)]
pub enum EventQuery {
    /// find a query or queries
    Find {
        /// 1 or more event queries to apply in order
        queries: Vec<FindEventNode>
    },
    // TODO
    // EXISTS - is there a single event that matches the params
    // COUNT - just display how many events there are that match this
}

/// used to filter events
#[derive(Debug)]
pub enum EventNameFilter {
    /// any filter
    Any,
    /// filter just be of type name
    Named(String),
}

/// finds a
#[derive(Debug)]
pub struct FindEventNode {
    /// events to look for
    pub(crate) event_type: EventNameFilter,
    /// predicates to apply to each event
    pub(crate) operator: Operator,
}

#[derive(Debug)]
pub struct PropPath {
    /// a basic dot separated path  
    pub(crate) segments: Vec<String>
}

impl PropPath {
    pub fn lookup_value<'ev>(&self, event: &'ev Event) -> Result<&'ev AbstractValue, EventFilterError> {
        let mut current_prop_map = Some(event.event_obj().props());
        let mut current_value: Option<&AbstractValue> = None;
        for segment in &self.segments {
            
            if current_prop_map.is_none() {
                // we can't keep going...
                return Err(EventFilterError::KeyNotFound)
            }
            
            match current_prop_map.unwrap().get(segment) {
                None => {
                    // if this key doesn't exist then we're done
                    return Err(EventFilterError::KeyNotFound)
                }
                Some(value) => {
                    current_value = Some(value);
                    match value {
                        AbstractValue::Object(obj) => {
                            current_prop_map = Some(obj.props());
                        }
                        AbstractValue::Map(map) => {
                            current_prop_map = Some(map)
                        }
                        _ => current_prop_map = None
                    }
                }
            }
        }
        
        current_value.ok_or(EventFilterError::KeyNotFound)
    } 
}

/// different operators we can perform on queries
#[derive(Debug)]
pub enum Operator {
    /// determines if a field is equal to the given value
    Eq {
        prop_name: PropPath,
        comparison: String,
    },
    /// determines if an event has this property
    Has(PropPath),
    /// determines if an event came from a server
    Server(String),
    /// find all events that are after the given event id
    After(usize),
    /// find all events before the given event id
    Before(usize),

    /// negates an operator
    Not(Box<Operator>),
    /// conjunction of operators
    And(Vec<Operator>),
    /// disjunction of operators
    Or(Vec<Operator>)
}
