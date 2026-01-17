use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, ListItem, ListState, List},
};

use crate::app::App;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}
impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn draw(f: &mut Frame, area: Rect, items: Vec<ListItem>, app: &mut App) {
        let list = List::new(items)
            .block(Block::bordered().title("List"))
            .highlight_style(Style::new().on_blue());
        f.render_stateful_widget(list, area, &mut app.networks.state);
    }
}

