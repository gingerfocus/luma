pub mod mode;
pub mod sync;

use indexmap::IndexMap;
use tui::style::Color;

pub type Luma = IndexMap<String, Vec<Link>>;

#[derive(Default)]
pub struct State {
    pub selected_tab: usize,
    pub selected_index: usize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct OpenCommand {
    pub cmd: &'static str,
    pub args: Box<[&'static str]>,
}

impl OpenCommand {
    pub fn open(&self, name: &str) {
        // it isn't really out concern right now how the process went
        _ = std::process::Command::new(self.cmd)
            .args(self.args.iter())
            .arg(name)
            // .stdout(Stdio::piped())
            // .stderr(Stdio::piped())
            .output();
        // .spawn();
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Link {
    pub name: String,
    pub link: String,
    pub file: Option<String>,
    pub desc: Option<String>,
    pub artist: Option<String>,
    pub color: Option<Color>,
}

impl Link {
    pub fn new(name: impl Into<String>, link: impl Into<String>) -> Link {
        Link {
            name: name.into(),
            link: link.into(),
            file: None,
            desc: None,
            artist: None,
            color: None,
        }
    }

    pub fn stub() -> Link {
        Default::default()
    }
}
