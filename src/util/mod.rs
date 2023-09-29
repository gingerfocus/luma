#[macro_use]
pub mod globals;

use futures::executor::block_on;

use crate::prelude::*;

pub async fn get_tab_and_index() -> (usize, usize) {
    let state = STATE.read().await;
    (state.selected_tab, state.selected_index)
}

pub fn blocking_get_tab_and_index() -> (usize, usize) {
    let state = block_on(async { STATE.read().await });
    (state.selected_tab, state.selected_index)
}
