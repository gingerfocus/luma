use tui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

use super::State;

use crate::Luma;

pub fn draw(f: &mut Frame<'_>, luma: &Luma, stat: &State) {
    super::normal::draw(f, luma, stat);
    let fbox = float_box(f.size());

    f.render_widget(Clear, fbox);

    let p = Paragraph::new("Delete? (y/N)")
        .style(Style::default().fg(Color::Red))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Delete")
                .border_style(Style::default().fg(Color::Red)),
        )
        .alignment(tui::layout::Alignment::Center);

    f.render_widget(p, fbox);
}

fn float_box(wind: Rect) -> Rect {
    let center = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ],
    )
    .split(wind)[1];

    Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ],
    )
    .split(center)[1]
}
