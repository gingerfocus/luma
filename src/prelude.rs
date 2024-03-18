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

// use crate::state::OpenCommand;
// pub const LINK_OPENER: OpenCommand<1> = OpenCommand::new("firefox", ["--private-window"]);
// pub const FILE_OPENER: OpenCommand<0> = OpenCommand::new("mpv", []);
// pub const DOWNLOAD_DIR: &str = concat!(env!("HOME"), "/dl");

#[allow(dead_code)]
pub fn default<T: Default>() -> T {
    T::default()
}
