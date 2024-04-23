//! A Collection of re-exports to make programing easier
#![allow(unused_imports)]

pub use crate::{
    //     app::model::Model,
    //     error::LumaError,
    //     event::Event,
    //     // input::Handler,
    state::{
        // sync::*,
        Luma,
        // Mode, State
    },
    //     util,
};
pub use resu::{Context, Report, Result, ResultExt};
pub use std::sync::mpsc;
pub use std::{env, fmt, fs, io};

use crate::state::OpenCommand;
pub const LINK_OPENER: OpenCommand = OpenCommand::new("brave", &[]);
// pub const FILE_OPENER: OpenCommand = OpenCommand::new("mpv", &[]);
pub const TEXT_OPENER: OpenCommand = OpenCommand::new("nvim", &[]);
// pub const DOWNLOAD_DIR: &str = concat!(env!("HOME"), "/dln");

#[allow(dead_code)]
pub fn default<T: Default>() -> T {
    T::default()
}
