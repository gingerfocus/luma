use std::path::PathBuf;

use tokio::task::JoinHandle;

use crate::prelude::*;

#[derive(Debug)]
pub enum LumaMessage {
    Redraw,
    Exit,
    SetMode(Mode),
    AddHandle(JoinHandle<Vec<LumaMessage>>),
    OpenEditor {
        text: String,
        resp: oneshot::Sender<String>,
    },
    // An empty path means save to the default path
    Save(Option<PathBuf>),
}

#[derive(Debug)]
pub enum ThreadMessage {
    Pause(oneshot::Sender<ThreadMessageResponse>),
    Resume,
    Close,
}

#[derive(Debug)]
pub enum ThreadMessageResponse {
    Ok, // (oneshot::Sender<EventReaderRequest>),
    No(String),
    Err(LumaError),
}
