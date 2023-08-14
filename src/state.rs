use tui::widgets::{Paragraph, Wrap};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct State {
    meta: Metadata,
    pub audios: Vec<Link>,
    pub videos: Vec<Link>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    download_dir: Option<String>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Link {
    pub name: String,
    link: String,
    file: Option<String>,
    desc: Option<String>,
    artist: Option<String>,
}

impl Link {
    pub fn as_paragraph(&self) -> tui::widgets::Paragraph {
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

        Paragraph::new(buf).wrap(Wrap { trim: false })
    }
}
