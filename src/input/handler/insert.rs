use crate::{input::Key, mode::InsertData, prelude::*};

use super::Handler;

pub fn add_all(h: &mut Handler) {
    h.add_insert_handler(Key::Enter, complete);
    h.add_insert_handlers([Key::Esc, Key::Ctrl('c')], cancel);
    h.add_insert_handler(Key::Backspace, delete);
}

fn delete(_state: &mut Luma, data: &mut InsertData) -> Option<LumaMessage> {
    data.buffers
        .get_mut(data.index)?
        .pop();
    Some(LumaMessage::Redraw)
}

fn complete(state: &mut Luma, data: &mut InsertData) -> Option<LumaMessage> {
    Some(data.next_or_destructure().and_then(|(buffers, callback)| {
            callback(state, buffers);
            Some(LumaMessage::SetMode(Mode::Normal))
    }).unwrap_or(LumaMessage::Redraw))
}

fn cancel(_state: &mut Luma, _data: &mut InsertData) -> Option<LumaMessage> {
    Some(LumaMessage::SetMode(Mode::Normal))
}
