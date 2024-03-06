use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::ExitCode;
use log::{LevelFilter, warn};
use crate::err::AppError;
use crate::event::{Event, EventParser};

mod event;
mod err;
mod abstract_object;

fn parse_event_log(file: File) -> Result<Vec<Event>, AppError> {
    let reader = BufReader::new(file);
    let mut events: Vec<Event> = Vec::new();
    let event_parser = EventParser::new();
    for line in reader.lines() {
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


fn main() -> Result<ExitCode, Box<dyn Error>> {

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();
    
    let events = parse_event_log(File::open("/home/charlie/Programming/cs505-spring-2024/test-16-logs.txt")?)?;
    for event in &events {
        println!("{:?}", event)
    }
    
    println!("Parsed {} events", events.len());

    Ok(ExitCode::SUCCESS)
}
