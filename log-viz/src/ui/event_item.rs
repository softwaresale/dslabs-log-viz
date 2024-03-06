use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use dslabs_events::event::Event;

pub struct EventItemWidget<'item> {
    /// the event we are going to display
    event: &'item Event,
    idx: usize,
    selected: bool,
}

impl<'item> EventItemWidget<'item> {
    pub fn new(event: &'item Event, idx: usize) -> Self {
        Self { event, idx, selected: false, }
    }
    
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    
    fn compute_style(&self) -> Style {
        if self.selected {
            Style::new().fg(Color::Green)
        } else {
            Style::new().fg(Color::Gray)
        }
    }
}

impl<'item> Widget for EventItemWidget<'item> {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        // main vertical layout
        
        let containing_block = Block::default()
            .borders(Borders::all())
            .border_style(self.compute_style());

        let main_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
            .spacing(1)
            .split(containing_block.inner(area));
        
        // render the top bar layout
        // [ info | timestamp | causing node ]
        {
            let top_bar_layout = Layout::horizontal([Constraint::Ratio(1, 3); 3])
                .spacing(1)
                .split(main_layout[0]);

            Paragraph::new(self.event.level().as_ref())
                .style(Color::Yellow)
                .render(top_bar_layout[0], buf);

            // TODO the rest
            Paragraph::new(format!("{}", self.idx))
                .right_aligned()
                .style(if self.selected {
                    Style::new().fg(Color::White)
                } else {
                    Style::new().fg(Color::Gray)
                })
                .render(top_bar_layout[2], buf);
        }

        // render the type of event
        {
            let node_ev = self.event.ev();

            Paragraph::new(node_ev.type_str())
                .render(main_layout[1], buf);
            
            Paragraph::new(format!("{}(...)", node_ev.payload().name()))
                .render(main_layout[2], buf);
        }
        
        containing_block.render(area, buf);
    }
}
