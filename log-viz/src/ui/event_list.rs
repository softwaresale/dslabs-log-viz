use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{ListState, StatefulWidget, Widget};
use dslabs_events::event::Event;
use crate::ui::event_item::EventItemWidget;

pub struct EventListWidget<'item> {
    /// all items we have
    items: &'item [Event],
}

impl<'item> EventListWidget<'item> {
    pub fn new(items: &'item [Event]) -> Self {
        Self { items }
    }
}

impl<'item> StatefulWidget for EventListWidget<'item> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let event_count = (area.height / 8) as usize;
        let start_offset = state.offset();
        let end_offset = start_offset + event_count;
        let selected_idx = state.selected();
        
        let (start_offset, end_offset) = if selected_idx.is_some_and(|idx| idx >= end_offset)  {
            *state.offset_mut() = end_offset;
            (end_offset, end_offset + event_count)
        } else if selected_idx.is_some_and(|idx| idx < start_offset) {
            *state.offset_mut() = start_offset - event_count;
            (state.offset(), state.offset() + event_count)
        } else {
            (start_offset, end_offset)
        };

        let layout = Layout::vertical(vec![Constraint::Ratio(1, event_count as u32); event_count])
            .spacing(1)
            .split(area);

        for (idx, displayed_items) in self.items[start_offset..end_offset].into_iter()
            .enumerate()
            .map(|(idx, event)| {
                let ev = EventItemWidget::new(event, start_offset + idx)
                    .selected(selected_idx.is_some_and(|sel| sel == (start_offset + idx)));
                (idx, ev)
            }) {

            displayed_items.render(layout[idx], buf);
        }
    }
}
