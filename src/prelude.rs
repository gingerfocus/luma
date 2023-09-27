//! A Collection of re-exports to make programing easier

pub use crate::{
    app::App,
    input::Event,
    luma::{Link, Luma},
    mode::Mode,
    ui::screen::Screen,
    LumaMessage,
};

pub use std::{fs, io};

use std::sync::{Arc, RwLock};
pub type GlobalState = Arc<RwLock<State>>;

#[derive(Default)]
pub struct State {
    pub selected_tab: usize,
    pub selected_index: usize,
}

lazy_static::lazy_static! {
    pub static ref STATE: GlobalState = Default::default();
}

pub type Result<T> = core::result::Result<T, LumaError>;

#[derive(thiserror::Error, Debug)]
pub enum LumaError {
    #[error("File io failure")]
    Io(#[from] io::Error),

    #[error("Generic error: `{0}`")]
    Generic(String),

    #[error("Config parse error: {0}")]
    Config(#[from] json::Error),

    #[error("Encoder error: {0}")]
    Encoder(#[from] vorbis_rs::VorbisError),

    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },

    #[error("Unknown error")]
    Unknown,
    // Mpd,
}
