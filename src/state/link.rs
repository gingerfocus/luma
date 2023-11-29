use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct GlobalLink(Arc<RwLock<Link>>);

impl PartialEq for GlobalLink {
    fn eq(&self, other: &Self) -> bool {
        &self.0.read().unwrap() as &Link == &other.0.read().unwrap() as &Link
    }
}
impl Eq for GlobalLink {}
impl PartialOrd for GlobalLink {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl Ord for GlobalLink {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub name: String,
    pub link: String,
    pub file: Option<String>,
    pub desc: Option<String>,
    pub artist: Option<String>,
    pub color: Option<String>,
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

    pub fn as_global(self) -> GlobalLink {
        GlobalLink(Arc::new(RwLock::new(self)))
    }

    pub fn stub() -> Link {
        Default::default()
    }
}
