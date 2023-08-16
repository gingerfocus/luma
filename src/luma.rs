use crate::ui::screen::ScreenType;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Luma {
    pub meta: Metadata,
    pub audios: Vec<Link>,
    pub reading: Vec<Link>,
}

impl Luma {
    // TODO: convert this to Vec<&Link>
    pub fn set(&self, tab: ScreenType) -> &Vec<Link> {
        match tab {
            ScreenType::Audio => self.audios.as_ref(),
            ScreenType::Reading => self.reading.as_ref(),
        }
    }
    pub fn set_mut(&mut self, tab: ScreenType) -> &mut Vec<Link> {
        match tab {
            ScreenType::Audio => &mut self.audios,
            ScreenType::Reading => &mut self.reading,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Metadata {
    downloads: DownloadInfo,
    pub audio_open: OpenCommand,
    reading_open: OpenCommand,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
struct DownloadInfo {
    dir: String,
    audio_type: Option<String>,
    video_type: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct OpenCommand {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Link {
    pub name: String,
    pub link: String,
    pub file: Option<String>,
    pub desc: Option<String>,
    pub artist: Option<String>,
}

impl Link {
    pub fn new(name: impl Into<String>, link: impl Into<String>) -> Link {
        Link {
            name: name.into(),
            link: link.into(),
            file: None,
            desc: None,
            artist: None,
        }
    }
}
