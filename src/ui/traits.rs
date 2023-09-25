// use tui::prelude::*;
use tui::{
    style::Style,
    text::{Line, Span},
    widgets::*,
};

use crate::luma::Link;

pub trait AsParagraph {
    fn as_paragraph(&self) -> tui::widgets::Paragraph;
}

impl AsParagraph for Link {
    fn as_paragraph(&self) -> tui::widgets::Paragraph {
        let name = Some(format!("Name: {}", self.name));
        let link = Some(format!("Link: {}", self.link));
        let file = self.artist.as_ref().map(|f| format!("File: {}", f));
        let artist = self.artist.as_ref().map(|a| format!("Artist: {}", a));
        let desc = self.desc.as_ref().map(|d| format!("Description: {}", d));

        let mut buf = String::new();

        [name, link, file, artist, desc]
            .iter()
            .flatten()
            .for_each(|l| {
                buf.push_str(l);
                buf.push('\n');
            });

        tui::widgets::Paragraph::new(buf).wrap(Wrap { trim: false })
    }
}

pub trait AsListItem {
    fn as_list_item(&self) -> ListItem;
}

impl AsListItem for Link {
    fn as_list_item(&self) -> ListItem {
        ListItem::new(Line::from(vec![Span::raw(self.name.clone())]))
            .style(Style::new().fg(self.color.unwrap_or_default()))
    }
}
