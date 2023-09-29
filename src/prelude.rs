//! A Collection of re-exports to make programing easier

pub use crate::{
    app::App,
    error::LumaError,
    event::Event,
    input::Handler,
    mode::Mode,
    state::{Luma, State},
    ui::screen::Screen,
    util, LumaMessage,
};
pub use std::{fs, io};
pub use tokio::sync::{mpsc, oneshot};

pub type Result<T> = core::result::Result<T, LumaError>;

type GlobalState = Arc<RwLock<State>>;
type GlobalLuma = Arc<RwLock<Luma>>;
pub type GlobalMode = std::sync::Arc<std::sync::RwLock<Mode>>;

use std::sync::Arc;
use tokio::sync::RwLock;

lazy_static::lazy_static! {
    pub static ref STATE: GlobalState = Default::default();
    pub static ref LUMA: GlobalLuma = Default::default();

    pub static ref AUDIO_OPENER: crate::state::OpenCommand = crate::state::OpenCommand {
        cmd: "firefox",
        args: ["--private-window"].into(),
    };
}

// const LINK_OPENER: LazyCell<luma::OpenCommand> = LazyCell::new(|| luma::OpenCommand {
//     cmd: "firefox",
//     args: ["--private-window"].into(),
// });
// const TEXT_OPENER: LazyCell<luma::OpenCommand> = LazyCell::new(|| luma::OpenCommand {
//     cmd: "firefox",
//     args: ["--private-window"].into(),
// });
// const DOWNLOAD_DIR: &'static str = "~/dl";
