
/// top level query object
pub enum EventQuery<'query> {
    /// find a query or queries
    Find {
        /// 1 or more event queries to apply in order
        queries: Vec<FindEventNode<'query>>
    }
}

/// used to filter events
pub enum EventNameFilter<'query> {
    /// any filter
    Any,
    /// filter just be of type name
    Named(&'query str),
}

/// finds a
pub struct FindEventNode<'query> {
    /// events to look for
    pub(crate) event_type: EventNameFilter<'query>,
    /// predicates to apply to each event
    pub(crate) operator: Operator<'query>,
}

/// different operators we can perform on queries
#[derive(Debug)]
pub enum Operator<'query> {
    /// determines if a field is equal to the given value
    Eq {
        prop_name: &'query str,
        comparison: &'query str,
    },
    /// determines if an event has this property
    Has(&'query str),

    /// negates an operator
    Not(Box<Operator<'query>>),
    /// conjunction of operators
    And(Vec<Operator<'query>>),
    /// disjunction of operators
    Or(Vec<Operator<'query>>)
}
