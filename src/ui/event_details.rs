use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph, Widget};
use crate::ds_events::abstract_object::pretty_print::AbstractObjectPrettyPrinter;
use crate::ds_events::event::Event;

pub struct EventDetailsWidget<'ev> {
    event: &'ev Event,
}

impl<'ev> EventDetailsWidget<'ev> {
    pub fn new(event: &'ev Event) -> Self {
        Self { event }
    }
}

impl<'ev> Widget for EventDetailsWidget<'ev> {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let pretty_print = AbstractObjectPrettyPrinter::new(self.event.event_obj());
        let obj_details = format!("{}", pretty_print);
        Paragraph::new(obj_details)
            .block(
                Block::bordered()
                    .title("Event Details")
            )
            .render(area, buf)
    }
}
