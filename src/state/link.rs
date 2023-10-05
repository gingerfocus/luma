use crate::prelude::*;

use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;
use tui::style::Color;

use std::process::Command;
use std::process::Stdio;

#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Debug)]
pub struct OpenCommand<const N: usize> {
    pub name: &'static str,
    pub args: [&'static str; N],
}

impl<const N: usize> OpenCommand<N> {
    pub const fn new(name: &'static str, args: [&'static str; N]) -> Self {
        Self { name, args }
    }

    pub fn open(&self, name: &str) -> Result<JoinHandle<Vec<LumaMessage>>> {
        // it isn't really out concern right now how the process went

        let mut child = Command::new(self.name);
        child
            .args(self.args.iter())
            .arg(name)
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let h = tokio::spawn(async move {
            let mut child = child.spawn().unwrap();

            let _id = child.id();

            child.wait().unwrap();

            vec![]
        });

        Ok(h)
    }
}
