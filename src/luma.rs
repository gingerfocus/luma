use indexmap::IndexMap;
use tui::style::Color;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Luma {
    meta: Metadata,
    links: IndexMap<String, Vec<Link>>,
}

impl Luma {
    pub fn get_set_by_name(&self, tab: String) -> Option<&Vec<Link>> {
        self.links.get(&tab)
    }
    pub fn get_set_by_index(&self, index: usize) -> Option<&Vec<Link>> {
        self.links.get_index(index).map(|l| l.1)
    }
    pub fn get_mut_set_by_name(&mut self, tab: String) -> Option<&mut Vec<Link>> {
        self.links.get_mut(&tab)
    }
    pub fn get_mut_set_by_index(&mut self, index: usize) -> Option<&mut Vec<Link>> {
        self.links.get_index_mut(index).map(|x| x.1)
    }
    pub fn get_key_by_index(&self, index: usize) -> Option<String> {
        self.links.get_index(index).map(|x| x.0.clone())
    }

    // inserts the thing into the index
    pub fn insert_at_index(&mut self, tab: usize, index: Option<usize>, link: Link) {
        match self.get_mut_set_by_index(tab) {
            Some(set) => {
                if let Some(i) = index {
                    set.insert(i, link);
                }
            }
            None => {
                let name = format!("new set {}", self.get_num_sets());
                let set = vec![link];
                self.links.insert(name, set);
            }
        }
    }

    pub fn get_set_len_by_index(&self, index: usize) -> usize {
        self.get_set_by_index(index).map(Vec::len).unwrap_or(0)
    }

    pub fn get_num_sets(&self) -> usize {
        self.links.len()
    }
    pub fn get_set_names(&self) -> Vec<&String> {
        self.links.keys().collect()
    }

    pub fn get_opener(&self, file: impl AsRef<str>) -> Option<&OpenCommand> {
        self.meta.openers.get(file.as_ref())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Metadata {
    downloads: DownloadInfo,
    openers: HashMap<String, OpenCommand>,
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

impl OpenCommand {
    pub fn open(&self, name: &str) {
        // it isn't really out concern right now how the process went
        _ = std::process::Command::new(&self.cmd)
            .args(&self.args)
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
            color: None
        }
    }
}
