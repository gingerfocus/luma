use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs},
    Frame,
};

use crate::state::Link;
use tui::text::Text;

use super::State;

use crate::Luma;

pub fn draw(f: &mut Frame<'_>, luma: &Luma, stat: &State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(f.size());

    let tabb_pane = chunks[0];

    let div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Min(1)])
        .split(chunks[1]);

    let list_pane = div[0];
    let view_pane = div[1];

    // let mut g = 0u8;
    // let mut r = 255u8;
    // for col in view_pane.rows() {
    //     let a = Block::new().bg(Color::Rgb(r, g, 0));
    //     f.render_widget(a, col);
    //     g = g.overflowing_add(4).0;
    //     r = r.overflowing_sub(4).0;
    // }

    let tabs = Tabs::new(luma.tabs.iter().map(|l| l.0.as_str()))
        .select(stat.tabb)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(symbols::DOT);
    f.render_widget(tabs, tabb_pane);

    if let Some((_, items)) = luma.tabs.get(stat.tabb) {
        let joined_border_set = symbols::border::Set {
            top_right: symbols::line::ROUNDED.horizontal_down,
            bottom_right: symbols::line::ROUNDED.horizontal_up,
            ..symbols::border::ROUNDED
        };
        let list = List::new(items.iter().map(as_list_item))
            .block(
                Block::new()
                    .border_set(joined_border_set)
                    .borders(Borders::all()),
            )
            .highlight_style(Style::default().bg(Color::Red));

        let mut state = ListState::default()
            .with_selected(Some(stat.selc))
            .with_offset(stat.ofst);

        f.render_stateful_widget(list, list_pane, &mut state);

        if let Some(item) = items.get(stat.selc) {
            let view = as_paragraph(item).block(
                Block::new()
                    .border_set(symbols::border::ROUNDED)
                    .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM),
            );
            f.render_widget(view, view_pane);
        }
    }
}

fn as_list_item(link: &Link) -> ListItem<'static> {
    // as list item
    let t = Text::raw(link.name.clone());
    ListItem::new(t)
    // .style(Style::new().fg(self.color.unwrap_or_default()));
}

fn as_paragraph(link: &Link) -> Paragraph<'static> {
    let name = Some(format!("Name: {}", link.name));
    let l = Some(format!("Link: {}", link.link));
    let file = link.file.as_ref().map(|f| format!("File: {}", f));
    let artist = link.artist.as_ref().map(|a| format!("Artist: {}", a));
    let desc = link.desc.as_ref().map(|d| format!("Description: {}", d));

    let mut buf = String::new();

    [name, l, file, artist, desc]
        .into_iter()
        .flatten()
        .for_each(|l| {
            buf.push_str(&l);
            buf.push('\n');
        });

    Paragraph::new(buf)
}

// pub fn float_box(wind: Rect) -> Rect {
//     let center = Layout::new(
//         Direction::Horizontal,
//         [
//             Constraint::Percentage(10),
//             Constraint::Percentage(80),
//             Constraint::Percentage(10),
//         ],
//     )
//     .split(wind)[1];
//
//     Layout::new(
//         Direction::Vertical,
//         [
//             Constraint::Percentage(10),
//             Constraint::Percentage(80),
//             Constraint::Percentage(10),
//         ],
//     )
//     .split(center)[1]
// }

// pub fn prompt(msg: &str) -> Paragraph<'_> {
//     Paragraph::new(msg)
//         .block(
//             Block::default()
//                 .title("Prompt")
//                 .borders(Borders::all())
//                 .border_type(BorderType::Rounded),
//         )
//         .alignment(Alignment::Left)
// }
//
// pub fn input(msg: &str) -> Paragraph<'_> {
//     Paragraph::new(msg).block(
//         Block::default()
//             .title("Input")
//             .borders(Borders::all())
//             .border_type(BorderType::Rounded),
//     )
//     // .alignment(Alignment::Left);
// }
