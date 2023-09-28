use crate::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum LumaError {
    #[error("File io failure")]
    Io(#[from] io::Error),

    #[error("Generic error: `{0}`")]
    Generic(String),

    #[error("Config parse error: {0}")]
    Config(#[from] json::Error),

    // #[error("Encoder error: {0}")]
    // Encoder(#[from] vorbis_rs::VorbisError),

    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    #[error("Unknown error")]
    Unknown,
    // Mpd,
}
