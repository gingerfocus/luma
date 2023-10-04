use crate::state::link::Link;
use tui::{prelude::*, widgets::*};

pub fn float_box(msg: &str, width: u16, height: u16) -> Rect {
    let new_width = msg.lines().map(|l| l.len()).max().unwrap() as u16 + 2;
    let new_height = msg.lines().count() as u16 + 2;

    let x = (width - new_width) / 2;
    let y = (height - new_height) / 2;

    Rect {
        x,
        y,
        width: new_width,
        height: new_height,
    }
}

pub fn list(set: &[Link]) -> List<'_> {
    use super::traits::AsListItem;
    List::new(
        set.iter()
            .map(AsListItem::as_list_item)
            .collect::<Vec<ListItem>>(),
    )
    // .highlight_symbol(">")
    .highlight_style(Style::default().bg(Color::Red))
    .block(Block::default().title("links").borders(Borders::all()))
}

pub fn tabs<'a>(names: impl IntoIterator<Item = &'a String>, selected: usize) -> Tabs<'static> {
    // NOTE: most of this is just constant so try using a lazy static

    Tabs::new(
        names
            .into_iter()
            .cloned()
            .map(Line::from)
            .collect::<Vec<Line<'_>>>(),
    )
    .select(selected)
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().fg(Color::Yellow))
    .divider(symbols::DOT)
}

// this meathod will always sucseed but wont be called on all renders
pub fn preview(l: &Link) -> Paragraph<'_> {
    use super::traits::AsParagraph;

    l.as_paragraph()
        .block(Block::default().title("Info").borders(Borders::all()))
}

pub fn prompt(msg: &str) -> Paragraph<'_> {
    Paragraph::new(msg)
        .block(
            Block::default()
                .title("Prompt")
                .borders(Borders::all())
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Left)
}

pub fn input(msg: &str) -> Paragraph<'_> {
    Paragraph::new(msg).block(
        Block::default()
            .title("Input")
            .borders(Borders::all())
            .border_type(BorderType::Rounded),
    )
    // .alignment(Alignment::Left);
}
