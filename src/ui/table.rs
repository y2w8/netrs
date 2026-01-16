use ratatui::{Frame, layout::{Constraint, Rect}, style::Style, widgets::{Block, Table, Row}};

use crate::app::App;

pub fn draw(f: &mut Frame, area: Rect, app: &mut App) {
    let rows = vec![
        Row::new(vec!["Enter", "blabla"])
    ];
    let table = Table::new(rows, &[Constraint::Percentage(100)])
        .block(Block::bordered().title_top("Networks"))
        .row_highlight_style(Style::new().on_blue().black());
    f.render_widget(table, area);
}
