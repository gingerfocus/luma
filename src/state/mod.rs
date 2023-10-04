pub mod link;
pub mod mode;
pub mod sync;

use indexmap::IndexMap;

use self::link::Link;

pub type Luma = IndexMap<String, Vec<Link>>;

#[derive(Default)]
pub struct State {
    pub selected_tab: usize,
    pub selected_index: usize,
    pub unsaved_changes: bool,
}
