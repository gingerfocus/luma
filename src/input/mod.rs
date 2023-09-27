mod event;
mod handler;
mod key;
mod mouse;

use crate::prelude::*;

pub use self::event::Event;
pub use self::handler::Handler;
pub use self::key::Key;
pub use self::mouse::Mouse;
pub use self::mouse::MouseKind;

pub fn handle(
    event: Event,
    screen: &mut Screen,
    luma: &mut Luma,
    mode: &mut Mode,
    handler: &Handler,
) -> Option<LumaMessage> {
    match event {
        Event::GainedFocus(_did) => None,
        Event::Input(key) => handler.handle(key, luma, mode),
        Event::Click(click) => handle_click(screen, click, luma, mode),
        Event::Paste(paste) => handle_paste(screen, paste, luma, mode),
        Event::Resize(_x, _y) => None,
        Event::Tick => None,
    }
}

fn handle_paste(
    _screen: &Screen,
    paste: String,
    _luma: &mut Luma,
    mode: &mut Mode,
) -> Option<LumaMessage> {
    match mode {
        Mode::Normal => None,
        Mode::Prompt { .. } => None,
        Mode::Insert(data) => {
            data.buffers
                .get_mut(data.index)
                .expect("something has gone very wrong for you to get here")
                .push_str(&paste);
            None
        }
    }
}

fn handle_click(
    _screen: &mut Screen,
    click: Mouse,
    _luma: &mut Luma,
    _mode: &mut Mode,
) -> Option<LumaMessage> {
    let Mouse { kind, .. } = click;
    match kind {
        crate::input::MouseKind::LeftClick => todo!(),
        crate::input::MouseKind::RightClick => todo!(),
        crate::input::MouseKind::MiddleClick => todo!(),
        crate::input::MouseKind::Drag => todo!(),
        crate::input::MouseKind::Scroll(i) => {
            let mut gwrite = STATE.write().unwrap();
            if i < 0 {
                _ = gwrite.selected_index.saturating_sub(-i as usize);
            } else {
                gwrite.selected_index += i as usize;
                // TODO: check upper bounds on index
            }
            Some(LumaMessage::Redraw)
        }
        crate::input::MouseKind::Nothing => None,
    }
}
