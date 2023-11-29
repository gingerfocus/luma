//! A Collection of re-exports to make programing easier

pub use crate::{
    app::model::Model,
    error::LumaError,
    event::Event,
    // input::Handler,
    state::{sync::*, Luma, Mode, State},
    util,
};
pub use futures::executor::block_on;
pub use std::{env, fs, io};
pub use tokio::sync::{mpsc, oneshot};

pub type Result<T> = core::result::Result<T, LumaError>;

use std::sync::Arc;
use tokio::sync::RwLock;
lazy_static::lazy_static! {
    pub static ref STATE: Arc<RwLock<State>> = Default::default();
    pub static ref LUMA: Arc<RwLock<Luma>>= Default::default();
}

use crate::state::OpenCommand;
pub const LINK_OPENER: OpenCommand<1> = OpenCommand::new("firefox", ["--private-window"]);
pub const FILE_OPENER: OpenCommand<0> = OpenCommand::new("mpv", []);
pub const DOWNLOAD_DIR: &str = concat!(env!("HOME"), "/dl");

pub fn default<T: Default>() -> T {
    T::default()
}
