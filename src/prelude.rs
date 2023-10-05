//! A Collection of re-exports to make programing easier

pub use crate::{
    app::App,
    error::LumaError,
    event::Event,
    input::Handler,
    state::{mode::Mode, sync::*, Luma, State},
    ui::screen::Screen,
    util,
};
pub use futures::executor::block_on;
pub use std::{env, fs, io};
pub use tokio::sync::{mpsc, oneshot};

pub type Result<T> = core::result::Result<T, LumaError>;

use std::sync::Arc;
use tokio::sync::RwLock;

type GlobalState = Arc<RwLock<State>>;
type GlobalLuma = Arc<RwLock<Luma>>;
pub type GlobalMode = Arc<RwLock<Mode>>;

use crate::state::link::OpenCommand;

lazy_static::lazy_static! {
    pub static ref STATE: GlobalState = Default::default();
    pub static ref LUMA: GlobalLuma = Default::default();
}

pub const LINK_OPENER: OpenCommand<1> = OpenCommand::new("firefox", ["--private-window"]);
pub const FILE_OPENER: OpenCommand<0> = OpenCommand::new("mpv", []);
pub const DOWNLOAD_DIR: &str = concat!(env!("HOME"), "/dl");

pub fn default<T: Default>() -> T {
    T::default()
}
