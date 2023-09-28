#[macro_export]
macro_rules! state_mut {
    () => {
        &mut STATE.write().unwrap() as &mut State
    };
}
