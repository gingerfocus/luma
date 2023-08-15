#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct State {
    pub meta: Metadata,
    pub audios: Vec<Link>,
    pub reading: Vec<Link>,
}

impl State {
    pub fn set(&self, index: usize) -> &Vec<Link> {
        match index {
            0 => &self.audios,
            1 => &self.reading,
            i => panic!("invalid index: {}", i),
        }
    }
    pub fn set_mut(&mut self, index: usize) -> &mut Vec<Link> {
        match index {
            0 => &mut self.audios,
            1 => &mut self.reading,
            i => panic!("invalid index: {}", i),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    downloads: DownloadInfo,
    pub audio_open: OpenCommand,
    reading_open: OpenCommand,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DownloadInfo {
    dir: String,
    audio_type: Option<String>,
    video_type: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
    pub fn new(name: String, link: String) -> Link {
        Link {
            name,
            link,
            file: None,
            desc: None,
            artist: None,
        }
    }
}
