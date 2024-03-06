pub mod node_ev;
mod parser;

use std::cell::Cell;
use regex::Regex;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::PrimitiveDateTime;
use crate::err::AppError;
use crate::event::node_ev::{NodeEvent};

#[derive(Debug)]
pub enum EventLevel {
    Finer,
    Finest,
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

fn parse_timestamp(timestamp: &str) -> Result<PrimitiveDateTime, AppError> {
    let format = format_description!("[year]-[month padding:zero repr:numerical]-[day padding:zero] [hour]:[minute padding:zero]:[second padding:zero]");
    let result = PrimitiveDateTime::parse(timestamp, format);
    return result.map_err(|_| AppError::new("All timestamps should be the same format"));
}

#[derive(Debug)]
pub struct Event {
    id: usize,
    level: EventLevel,
    time: PrimitiveDateTime,
    ev: NodeEvent
}

impl Event {
    pub fn new(id: usize, level: EventLevel, time: PrimitiveDateTime, ev: NodeEvent) -> Self {
        Self { id, level, time, ev }
    }


    pub fn level(&self) -> &EventLevel {
        &self.level
    }
    pub fn time(&self) -> PrimitiveDateTime {
        self.time
    }
    pub fn ev(&self) -> &NodeEvent {
        &self.ev
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
            line_regex: Regex::new(r"^\[(\w+)\s*] \[([^]]+)] \[[^]]+] (.+)$").expect("Regex compilation should not fail"),
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

            let msg_info = result.get(3).unwrap().as_str();
            let node_ev = NodeEvent::parse(msg_info)?;

            let id = self.running_id.get();
            self.running_id.set(id + 1);
            
            Ok(Event::new(id, log_level, ts, node_ev))
        } else {
            Err(AppError::new("Line was of an invalid format"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::event::EventParser;

    #[test]
    fn parse_custom_event() {
        let event_line = "[INFO   ] [2024-03-05 22:59:25] [dslabs.paxos.PaxosServer] CommandExecuted(-> server2, PaxosSlotEntry(amoCommand=AMOCommand(command=KVStore.Put(key=client5-5, value=7uocFqRu), address=client5, sequenceNum=72), slotStatus=CHOSEN, isExecuted=true, acceptedBallot=Ballot(serverAddress=server1, roundNum=1), acceptors=[server2, server1]))";
        let _event = EventParser::new()
            .parse(event_line).expect("Parsing should not fail");
    }
}
