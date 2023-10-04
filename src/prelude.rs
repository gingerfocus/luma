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

lazy_static::lazy_static! {
    pub static ref STATE: GlobalState = Default::default();
    pub static ref LUMA: GlobalLuma = Default::default();
}

use crate::state::link::OpenCommand;

pub const AUDIO_OPENER: OpenCommand<1> = OpenCommand::new("firefox", ["--private-window"]);

// const LINK_OPENER: LazyCell<luma::OpenCommand> = LazyCell::new(|| luma::OpenCommand {
//     cmd: "firefox",
//     args: ["--private-window"].into(),
// });
// const TEXT_OPENER: LazyCell<luma::OpenCommand> = LazyCell::new(|| luma::OpenCommand {
//     cmd: "firefox",
//     args: ["--private-window"].into(),
// });
// const DOWNLOAD_DIR: &'static str = "~/dl";
