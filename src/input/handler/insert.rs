use crate::{input::Key, mode::InsertData, prelude::*};

use super::Handler;

pub fn add_all(h: &mut Handler) {
    h.add_insert_handler(Key::Enter, complete);
    h.add_insert_handlers([Key::Esc, Key::Ctrl('c')], cancel);
    h.add_insert_handler(Key::Backspace, delete);
}

fn delete(data: &mut InsertData) -> Option<LumaMessage> {
    data.last_mut().unwrap().1.pop();
    LumaMessage::Redraw.into()
}

fn complete(data: &mut InsertData) -> Option<LumaMessage> {
    if let Some((_prompt, buffer, resp)) = data.pop() {
        resp.send(buffer).unwrap();

        LumaMessage::Redraw.into()
    } else {
        LumaMessage::SetMode(Mode::Normal).into()
    }
}

fn cancel(_data: &mut InsertData) -> Option<LumaMessage> {
    LumaMessage::SetMode(Mode::Normal).into()
}
