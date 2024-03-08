
use std::fs::File;
use std::io::{BufRead, BufReader};
use log::warn;
use crate::ds_events::err::AppError;
use crate::ds_events::event::{Event, EventParser};

pub mod event;
pub mod err;
pub mod abstract_object;

pub fn parse_event_log(file: File) -> Result<Vec<Event>, AppError> {
    let reader = BufReader::new(file);
    let mut events: Vec<Event> = Vec::new();
    let event_parser = EventParser::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line
            .map_err(|line_err| AppError::new(format!("line with error: {}", line_err)))?;

        match event_parser.parse(&line) {
            Ok(event) => events.push(event),
            Err(err) => {
                warn!("error occurred while parsing line: '{}'\nskipping: {}", line, err)
            }
        }
    }

    Ok(events)
}
