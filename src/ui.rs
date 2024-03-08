
pub mod event_list;

use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use ratatui::layout::{Constraint, Layout};

use crate::app::App;
use crate::ui::event_list::EventList;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    let starting_paragraph = Paragraph::new(format!("DSLabs Log Insight.\n\
         Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
         Press left and right to increment and decrement the counter respectively.\n\
         Current selected event: {}", app.event_list_state.selected_event)
    )
        .block(
            Block::bordered()
                .title("DS Labs Log Visualizer")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered();

    let layout = Layout::vertical([
        Constraint::Length(6),
        Constraint::Min(0)
    ])
        .spacing(2)
        .split(frame.size());

    frame.render_widget(
        starting_paragraph,
        layout[0],
    );
    
    let event_list = EventList::new(&app.events);
    frame.render_stateful_widget(event_list, layout[1], &mut app.event_list_state);
}
