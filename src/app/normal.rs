use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::Luma;

pub fn draw(f: &mut Frame<'_>, luma: &Luma) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
        .split(f.size());

    let tabb_pane = chunks[0];

    let div = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Min(1)].as_ref())
        .split(chunks[1]);

    let list_pane = div[0];
    let view_pane = div[1];

    let mut r = 0u8;
    let mut g = 255u8;
    for col in tabb_pane.columns() {
        let a = Block::new().bg(Color::Rgb(r, g, 0));
        f.render_widget(a, col);
        r = r.overflowing_add(2).0;
        g = g.overflowing_sub(1).0;
    }

    let audi = &luma.tabs[0].1;
    let list = List::new(audi.iter().map(as_list_item))
        .block(Block::new().borders(Borders::all()))
        .highlight_style(Style::default().bg(Color::Red));

    let mut state = ListState::default().with_selected(Some(1)).with_offset(0);

    f.render_stateful_widget(list, list_pane, &mut state);

    let a = as_paragraph(&audi[0]).block(Block::new().borders(Borders::all()));

    f.render_widget(a, view_pane);

    // let render = format!("{:?}", luma);
    // f.render_widget(Paragraph::new(render), area)
}

use crate::state::Link;
use tui::text::Text;

// impl Link {
//     /// Create a renderable explaination of the link
//     pub fn text(&self) -> String {
//     }
// }

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

// pub fn float_box(msg: &str, width: u16, height: u16) -> Rect {
//     let new_width = msg.lines().map(|l| l.len()).max().unwrap() as u16 + 2;
//     let new_height = msg.lines().count() as u16 + 2;
//
//     let x = (width - new_width) / 2;
//     let y = (height - new_height) / 2;
//
//     Rect {
//         x,
//         y,
//         width: new_width,
//         height: new_height,
//     }
// }
//
//
//
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
