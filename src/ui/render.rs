use crate::state::Link;
use tuirealm::tui::{style::Styled, text::Text, widgets::*};

// use tui::style::Style;

pub trait Render<T: Styled> {
    fn render(&self) -> T;
}

impl Link {
    /// Create a renderable explaination of the link
    pub fn text(&self) -> String {
        let name = Some(format!("Name: {}", self.name));
        let link = Some(format!("Link: {}", self.link));
        let file = self.file.as_ref().map(|f| format!("File: {}", f));
        let artist = self.artist.as_ref().map(|a| format!("Artist: {}", a));
        let desc = self.desc.as_ref().map(|d| format!("Description: {}", d));

        let mut buf = String::new();

        [name, link, file, artist, desc]
            .into_iter()
            .flatten()
            .for_each(|l| {
                buf.push_str(&l);
                buf.push('\n');
            });

        buf
    }
}

impl Render<ListItem<'static>> for Link {
    // pub fn list(set: &[Link]) -> List<'_>
    fn render(&self) -> ListItem<'static> {
        // as list item
        let t = Text::raw(self.name.clone());
        ListItem::new(t)
        // .style(Style::new().fg(self.color.unwrap_or_default()));
    }
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
