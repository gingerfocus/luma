use tui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs},
};

use crate::state::Link;
use tui::text::Text;

use super::State;

use crate::Luma;

pub fn draw(f: &mut Frame<'_>, luma: &Luma, state: &State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.size());

    let help_pane = chunks[1];
    help_barr(f, help_pane);

    main_pane(f, chunks[0], luma, state);
}

fn main_pane(f: &mut Frame<'_>, area: Rect, luma: &Luma, state: &State) {
    if luma.tabs.is_empty() {
        let p = Paragraph::new("No tabs to display. Try creating one with 'n'")
            .alignment(tui::layout::Alignment::Center)
            .block(Block::new().borders(Borders::all()));
        f.render_widget(p, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(area);

    let tabb_pane = chunks[0];
    tabb_barr(f, tabb_pane, luma, state);

    let disp_area = chunks[1];
    disp_pane(f, disp_area, luma, state);
}

fn error_pane(f: &mut Frame<'_>, area: Rect) {
    const ERROR_MSG: &str = "You should not be able to see this. If you are it would be nice if you could report it as a bug. Thanks!";
    let p = Paragraph::new(ERROR_MSG)
        .wrap(tui::widgets::Wrap { trim: false })
        .style(
            Style::default()
                .add_modifier(tui::style::Modifier::SLOW_BLINK)
                .fg(Color::Red),
        )
        .alignment(tui::layout::Alignment::Center)
        .block(Block::new().borders(Borders::all()));
    f.render_widget(p, area);
}

fn disp_pane(f: &mut Frame<'_>, area: Rect, luma: &Luma, state: &State) {
    if let Some((_, items)) = luma.tabs.get(state.tabb) {
        let div = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Min(1)])
            .split(area);

        let list_area = div[0];
        if items.is_empty() {
            let p = Paragraph::new("No links to display. Try creating a link with 'a'")
                .alignment(tui::layout::Alignment::Center)
                .block(Block::new().borders(Borders::all()));
            f.render_widget(p, list_area);
            return;
        } else {
            list_pane(f, list_area, items, state);
        }

        // let mut g = 0u8;
        // let mut r = 255u8;
        // for col in view_pane.rows() {
        //     let a = Block::new().bg(Color::Rgb(r, g, 0));
        //     f.render_widget(a, col);
        //     g = g.overflowing_add(4).0;
        //     r = r.overflowing_sub(4).0;
        // }

        let view_pane = div[1];
        if let Some(item) = items.get(state.selected) {
            prev_pane(f, view_pane, item);
        } else {
            error_pane(f, view_pane);
        }
    } else {
        error_pane(f, area);
    }
}

fn prev_pane(f: &mut Frame<'_>, area: Rect, link: &Link) {
    let view = as_paragraph(link).block(
        Block::new()
            .border_set(symbols::border::ROUNDED)
            .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM),
    );
    f.render_widget(view, area);
}

fn list_pane(f: &mut Frame<'_>, area: Rect, items: &[Link], state: &State) {
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

    let mut list_stat = ListState::default()
        .with_selected(Some(state.selected))
        .with_offset(state.ofst);

    f.render_stateful_widget(list, area, &mut list_stat);
}

fn tabb_barr(f: &mut Frame<'_>, area: Rect, luma: &Luma, state: &State) {
    let tabs = Tabs::new(luma.tabs.iter().map(|l| l.0.as_str()))
        .select(state.tabb)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(symbols::DOT);
    f.render_widget(tabs, area);
}

fn help_barr(f: &mut Frame<'_>, area: Rect) {
    let p = Paragraph::new(
        "Keys: q: quit, j: down, k: up, e: edit, o: open, d: delete, a: add, n: new tab, r: rename tab",
    );
    f.render_widget(p, area)
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
