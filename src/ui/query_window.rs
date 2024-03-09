use std::marker::PhantomData;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::StatefulWidget;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Widget};
use tui_textarea::TextArea;

pub struct QueryWindow<'a> {
    text_editor: PhantomData<TextArea<'a>>,
    selected: bool,
}

impl<'a> QueryWindow<'a> {
    pub fn new(selected: bool) -> Self {
        Self {
            text_editor: Default::default(),
            selected
        }
    }
}

impl<'a> StatefulWidget for QueryWindow<'a> {
    type State = TextArea<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::bordered()
            .border_style(if self.selected {
                Style::default().fg(Color::Blue)
            } else {
                Style::default()
            })
            .title("Query Editor");
        
        let block_area = block.inner(area);
        Widget::render(state.widget(), block_area, buf);
        block.render(area, buf);
    }
}
