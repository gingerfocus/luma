use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum LumaError {
    #[error("File io failure")]
    Io(#[from] io::Error),

    #[error("Terminal failure")]
    Terminal(#[from] tuirealm::terminal::TerminalError),

    #[error("Generic error: `{0}`")]
    Generic(String),

    #[error("Config parse error: {0}")]
    Config(#[from] json::Error),

    // #[error("Encoder error: {0}")]
    // Encoder(#[from] vorbis_rs::VorbisError),
    #[error("Threads failed to sync: {0}")]
    Sync(#[from] mpsc::error::SendError<ThreadMessage>),

    #[error("Thread paniced: {0}")]
    Thread(#[from] tokio::task::JoinError),

    #[error("Render failed: {0}")]
    Render(#[from] tuirealm::ApplicationError),

    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    #[error("Unknown error")]
    Unknown,
    // Mpd,
}

impl From<String> for LumaError {
    fn from(value: String) -> Self {
        LumaError::Generic(value)
    }
}

impl From<&str> for LumaError {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}
