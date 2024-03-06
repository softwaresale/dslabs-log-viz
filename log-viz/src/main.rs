use std::fs::File;
use log_viz::app::{App, AppResult};
use log_viz::event::{Event, EventHandler};
use log_viz::handler::handle_key_events;
use log_viz::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use dslabs_events::parse_event_log;

fn main() -> AppResult<()> {
    
    let events = parse_event_log(File::open("/home/charlie/Programming/cs505-spring-2024/test-16-logs.txt")?)?;
    
    // Create an application.
    let mut app = App::new(events);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
