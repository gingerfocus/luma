use crate::prelude::*;

#[macro_export]
macro_rules! state_mut {
    () => {
        &mut STATE.write().unwrap() as &mut State
    };
}

pub async fn get_tab_and_index() -> (usize, usize) {
    let state = STATE.read().await;
    (state.selected_tab, state.selected_index)
}
