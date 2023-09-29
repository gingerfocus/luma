use crate::{input::Key, mode::InsertData, prelude::*};

use super::Handler;

pub fn add_all(h: &mut Handler) {
    h.add_insert_handler(Key::Enter, complete);
    h.add_insert_handlers([Key::Esc, Key::Ctrl('c')], cancel);
    h.add_insert_handler(Key::Backspace, delete);
}

fn delete(data: &mut InsertData) -> Vec<LumaMessage> {
    data.last_mut().unwrap().1.pop();
    vec![LumaMessage::Redraw]
}

fn complete(data: &mut InsertData) -> Vec<LumaMessage> {
    if let Some((_prompt, buffer, resp)) = data.pop() {
        resp.send(buffer).unwrap();
        vec![LumaMessage::Redraw]
    } else {
        vec![LumaMessage::SetMode(Mode::Normal)]
    }
}

fn cancel(_data: &mut InsertData) -> Vec<LumaMessage> {
    // TODO: inform the call back it has been canceled
    vec![LumaMessage::SetMode(Mode::Normal)]
}
