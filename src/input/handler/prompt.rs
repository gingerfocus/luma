use crate::prelude::*;
use crate::{input::Key, mode::PromptData};

use super::Handler;

pub fn add_all(h: &mut Handler) {
    h.add_prompt_handlers([Key::Char('y'), Key::Char('Y')], accept);

    h.add_prompt_handlers([Key::Char('n'), Key::Char('N'), Key::Enter], decline);

    h.add_prompt_handlers([Key::Esc, Key::Char('q'), Key::Ctrl('c')], cancel);
}

fn accept(data: &mut PromptData) -> Option<LumaMessage> {
    if let Some(rx) = data.resp.take() {
        rx.send(crate::mode::PromptResponse::Yes).unwrap();
    }
    LumaMessage::SetMode(Mode::Normal).into()
}

fn decline(data: &mut PromptData) -> Option<LumaMessage> {
    if let Some(rx) = data.resp.take() {
        rx.send(crate::mode::PromptResponse::No).unwrap();
    }
    LumaMessage::SetMode(Mode::Normal).into()
}

fn cancel(data: &mut PromptData) -> Option<LumaMessage> {
    if let Some(rx) = data.resp.take() {
        rx.send(crate::mode::PromptResponse::Cancel).unwrap();
    }

    LumaMessage::SetMode(Mode::Normal).into()
}
