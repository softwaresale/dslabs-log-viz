use crate::ds_events::abstract_object::{AbstractObject, AbstractValue};
use crate::ds_events::err::AppError;
use crate::ds_events::event::parser::parse_node_event;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct MessageSendEvent {
    pub(crate) sender: String,
    pub(crate) dest: String,
    pub(crate) payload: AbstractObject,
}

impl MessageSendEvent {
    pub fn sender(&self) -> &str {
        &self.sender
    }
    pub fn dest(&self) -> &str {
        &self.dest
    }
    pub fn payload(&self) -> &AbstractObject {
        &self.payload
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct MessageReceiveEvent {
    pub(crate) sender: String,
    pub(crate) dest: String,
    pub(crate) payload: AbstractObject,
}

impl MessageReceiveEvent {
    pub fn sender(&self) -> &str {
        &self.sender
    }
    pub fn dest(&self) -> &str {
        &self.dest
    }
    pub fn payload(&self) -> &AbstractObject {
        &self.payload
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TimerSetEvent {
    pub(crate) dest: String,
    pub(crate) payload: AbstractObject,
}

impl TimerSetEvent {
    pub fn dest(&self) -> &str {
        &self.dest
    }
    pub fn payload(&self) -> &AbstractObject {
        &self.payload
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TimerDeliverEvent {
    pub(crate) dest: String,
    pub(crate) payload: AbstractObject,
}

impl TimerDeliverEvent {
    pub fn dest(&self) -> &str {
        &self.dest
    }
    pub fn payload(&self) -> &AbstractObject {
        &self.payload
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum CustomEventPayload {
    Object(AbstractObject),
    Message(String)
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CustomEventNode {
    pub(crate) name: String,
    pub(crate) sender: Option<String>,
    pub(crate) dest: String,
    pub(crate) payload: AbstractValue,
}

impl CustomEventNode {
    pub fn dest(&self) -> &str {
        &self.dest
    }
    
    pub fn sender(&self) -> Option<&str> {
        self.sender.as_ref().map(|s| s.as_str())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum NodeEvent {
    MsgSend(MessageSendEvent),
    MsgRecv(MessageReceiveEvent),
    TimerSet(TimerSetEvent),
    TimerRecv(TimerDeliverEvent),
    Custom(CustomEventNode)
}

impl NodeEvent {
    pub fn parse<InputT: AsRef<str>>(input: InputT) -> Result<Self, AppError> {
        parse_node_event(input.as_ref())
            .map(|(_, ev)| ev)
            .map_err(|parse_error| AppError::new(format!("failed to parse node event: {}", parse_error)))
    }

    pub fn sender(&self) -> Option<&str> {
        match self {
            NodeEvent::MsgSend(node) => Some(node.sender()),
            NodeEvent::MsgRecv(node) => Some(node.sender()),
            NodeEvent::Custom(node) => node.sender(),
            _ => None
        }
    }

    pub fn dest(&self) -> &str {
        match self {
            NodeEvent::MsgSend(node) => node.dest(),
            NodeEvent::MsgRecv(node) => node.dest(),
            NodeEvent::TimerSet(node) => node.dest(),
            NodeEvent::TimerRecv(node) => node.dest(),
            NodeEvent::Custom(node) => node.dest(),
        }
    }

    pub fn payload(&self) -> &AbstractObject {
        match self {
            NodeEvent::MsgSend(node) => node.payload(),
            NodeEvent::MsgRecv(node) => node.payload(),
            NodeEvent::TimerSet(node) => node.payload(),
            NodeEvent::TimerRecv(node) => node.payload(),
            NodeEvent::Custom(node) => match &node.payload {
                AbstractValue::Object(obj) => obj,
                _ => panic!("Non-object payload"),
            }
        }
    }

    pub fn type_str(&self) -> &str {
        match self {
            NodeEvent::MsgSend(_) => "Message Send",
            NodeEvent::MsgRecv(_) => "Message Recv",
            NodeEvent::TimerSet(_) => "Timer Set",
            NodeEvent::TimerRecv(_) => "Timer Recv",
            NodeEvent::Custom(node) => &node.name 
        }
    }
}
