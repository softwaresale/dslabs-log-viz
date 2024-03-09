
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

/// different operators we can perform on queries
#[derive(Debug)]
pub enum Operator {
    /// determines if a field is equal to the given value
    Eq {
        prop_name: String,
        comparison: String,
    },
    /// determines if an event has this property
    Has(String),

    /// negates an operator
    Not(Box<Operator>),
    /// conjunction of operators
    And(Vec<Operator>),
    /// disjunction of operators
    Or(Vec<Operator>)
}
