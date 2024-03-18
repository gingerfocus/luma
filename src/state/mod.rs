// pub mod data;
// pub mod sync;
mod link;
// mod mode;
// mod open;

pub use link::Link;
// pub use mode::Mode;
// pub use open::OpenCommand;

// pub type Set = Vec<Link>;

use indexmap::IndexMap;

pub type Luma = IndexMap<String, Vec<Link>>;

#[derive(Default)]
pub struct State {
    pub selected_tab: usize,
    pub selected_index: usize,
    pub unsaved_changes: bool,
}
