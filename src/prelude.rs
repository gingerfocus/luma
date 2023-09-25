pub use crate::{
    app::App,
    event::Event,
    luma::{Link, Luma},
    mode::Mode,
    ui::screen::Screen,
    util, LumaMessage,
};
pub use std::{fs, io};

pub type Result<T> = core::result::Result<T, LumaError>;

#[derive(thiserror::Error, Debug)]
pub enum LumaError {
    #[error("File io failure")]
    Io(#[from] io::Error),
    #[error("Generic error: `{0}`")]
    Generic(String),

    #[error("Config parse error: {0}")]
    Config(#[from] yaml::Error),

    #[error(transparent)]
    Static(#[from] anyhow::Error),

    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    #[error("Unknown error")]
    Unknown,
    // Mpd,
}
