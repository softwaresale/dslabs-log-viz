
pub mod event_list;
mod query_window;

use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use ratatui::layout::{Constraint, Layout};
use ratatui::text::Text;

use crate::app::App;
use crate::ui::event_list::EventList;
use crate::ui::query_window::QueryWindow;

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
        .spacing(1)
        .split(frame.size());

    frame.render_widget(
        starting_paragraph,
        layout[0],
    );
    
    let main_area_layout = Layout::horizontal([
        Constraint::Ratio(3, 4),
        Constraint::Ratio(1, 4)
    ])
        .spacing(1)
        .split(layout[1]);
    
    // render the event list
    let event_list = EventList::new(&app.events, app.filter_state.matching_events(), app.focused_window.is_event_list());
    frame.render_stateful_widget(event_list, main_area_layout[0], &mut app.event_list_state);

    let right_bar_layout = Layout::vertical([
        Constraint::Ratio(2, 3),
        Constraint::Ratio(1, 3)
    ])
        .spacing(1)
        .split(main_area_layout[1]);
    
    // render the query window and message area
    frame.render_stateful_widget(QueryWindow::new(app.focused_window.is_filter_list()), right_bar_layout[0], &mut app.query_text_area);

    let message_block = Block::bordered()
        .title("Messages");

    let message_area = message_block.inner(right_bar_layout[1]);
    let para = Paragraph::new(app.message_state.messages().join("\n"));
    frame.render_widget(para, message_area);
    frame.render_widget(message_block, right_bar_layout[1]);
}
