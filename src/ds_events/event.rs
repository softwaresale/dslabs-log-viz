pub mod node_ev;
mod parser;

use std::cell::Cell;
use regex::Regex;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::PrimitiveDateTime;
use crate::ds_events::abstract_object::AbstractObject;
use crate::ds_events::abstract_object::parser::parse_abstract_object;
use crate::ds_events::err::AppError;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum EventLevel {
    Finest,
    Finer,
    Info
}

impl TryFrom<&str> for EventLevel {
    type Error = AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "FINER" => Ok(Self::Finer),
            "FINEST" => Ok(Self::Finest),
            "INFO" => Ok(Self::Info),
            other => Err(AppError::new(format!("{} is not a valid EventLevel", other)))
        }
    }
}

impl AsRef<str> for EventLevel {
    fn as_ref(&self) -> &str {
        match self {
            EventLevel::Finer => "FINER",
            EventLevel::Finest => "FINEST",
            EventLevel::Info => "INFO",
        }
    }
}

#[derive(Debug)]
pub struct Event {
    /// the id for this event
    id: usize,
    /// the level associated with this event
    level: EventLevel,
    /// the time that this event occurred at
    time: PrimitiveDateTime,
    /// the address that originated this event
    originator: String,
    /// the event object associated with the event
    event_obj: AbstractObject,
}

impl Event {
    pub fn new<StrT: Into<String>>(id: usize, level: EventLevel, time: PrimitiveDateTime, originator: StrT, event_obj: AbstractObject) -> Self {
        Self {
            id,
            level,
            time,
            originator: originator.into(),
            event_obj
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn level(&self) -> &EventLevel {
        &self.level
    }
    pub fn time(&self) -> PrimitiveDateTime {
        self.time
    }
    pub fn originator(&self) -> &str {
        &self.originator
    }
    pub fn event_obj(&self) -> &AbstractObject {
        &self.event_obj
    }
}

pub struct EventParser<'a> {
    line_regex: Regex,
    timestamp_format: Vec<FormatItem<'a>>,
    running_id: Cell<usize>,
}

impl<'a> EventParser<'a> {
    pub fn new() -> Self {
        let format = format_description!("[year]-[month padding:zero repr:numerical]-[day padding:zero] [hour]:[minute padding:zero]:[second padding:zero]");
        let owned = format.to_owned();
        Self {
            line_regex: Regex::new(r"^\[(\w+)\s*] \[([^]]+)] \[[^]]+] ([\w\-]+): (.+)$").expect("Regex compilation should not fail"),
            timestamp_format: owned,
            running_id: Cell::new(0),
        }
    }

    pub fn parse<StrT: AsRef<str>>(&self, line: StrT) -> Result<Event, AppError> {
        if let Some(result) = self.line_regex.captures(line.as_ref()) {
            let log_level = result.get(1).unwrap().as_str();
            let log_level = EventLevel::try_from(log_level)?;

            let timestamp = result.get(2).unwrap().as_str();
            let ts = PrimitiveDateTime::parse(timestamp, &self.timestamp_format)
                .map_err(|err| AppError::new(format!("could not parse timestamp: {}", err)))?;

            let originator = result.get(3).unwrap().as_str();

            let payload_str = result.get(4).unwrap().as_str();
            let (_, payload) = parse_abstract_object(payload_str)
                .map_err(|_| AppError::new("Failed to parse abstract object"))?;

            let id = self.running_id.get();
            self.running_id.set(id + 1);
            
            Ok(Event::new(id, log_level, ts, originator, payload))
        } else {
            Err(AppError::new("Line was of an invalid format"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ds_events::event::{EventLevel, EventParser};

    #[test]
    fn event_level_order_correctly() {
        assert!(EventLevel::Finest < EventLevel::Finer);
        assert!(EventLevel::Finer < EventLevel::Info);
    }
    
    #[test]
    fn parse_custom_event() {
        let event_line = "[INFO   ] [2024-03-05 22:59:25] [dslabs.paxos.PaxosServer] server1: PaxosSlotEntry(amoCommand=AMOCommand(command=KVStore.Put(key=client5-5, value=7uocFqRu), address=client5, sequenceNum=72), slotStatus=CHOSEN, isExecuted=true, acceptedBallot=Ballot(serverAddress=server1, roundNum=1), acceptors=[server2, server1])";
        let _event = EventParser::new()
            .parse(event_line)
            .expect("Parsing should not fail");
    }
}
