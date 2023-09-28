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

pub fn handle(event: Event, handler: &mut Handler, mode: &mut Mode) -> Option<LumaMessage> {
    match event {
        Event::GainedFocus(_did) => None,
        Event::Input(key) => handler.handle(key, mode),
        Event::Click(click) => handle_click(click),
        Event::Paste(paste) => handle_paste(paste, mode),
        Event::Resize(_x, _y) => None,
        Event::Tick => None,
    }
}

fn handle_paste(paste: String, mode: &mut Mode) -> Option<LumaMessage> {
    match mode {
        Mode::Normal => None,
        Mode::Prompt { .. } => None,
        Mode::Insert(data) => {
            data.last_mut().unwrap().1.push_str(&paste);
            Some(LumaMessage::Redraw)
        }
    }
}

fn handle_click(click: Mouse) -> Option<LumaMessage> {
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
            }
            LumaMessage::Redraw.into()
        }
        crate::input::MouseKind::Nothing => None,
    }
}
