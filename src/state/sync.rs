use tokio::task::JoinHandle;

use crate::prelude::*;

pub enum LumaMessage {
    Redraw,
    Exit,
    SetMode(Mode),
    AddHandle(JoinHandle<Vec<LumaMessage>>),
    OpenEditor {
        text: String,
        resp: oneshot::Sender<String>,
    },
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
