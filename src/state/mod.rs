pub mod data;
pub mod sync;

mod link;
mod mode;
mod open;

use std::sync::{Arc, RwLock};

pub use link::{GlobalLink, Link};
pub use mode::Mode;
pub use open::OpenCommand;

pub type Set = Vec<Link>;
pub type GlobalSet = Arc<RwLock<Vec<GlobalLink>>>;

use indexmap::IndexMap;

pub type Luma = IndexMap<String, Vec<Link>>;
pub struct GlobalLuma(Arc<RwLock<IndexMap<String, GlobalSet>>>);

impl GlobalLuma {}

#[derive(Default)]
pub struct State {
    pub selected_tab: usize,
    pub selected_index: usize,
    pub unsaved_changes: bool,
}
