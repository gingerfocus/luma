use crate::event::key::Key;
use crate::prelude::*;
use crate::state::mode::{PromptData, PromptResponse};

use super::Handler;

pub fn add_all(h: &mut Handler) {
    h.add_prompt_handlers([Key::Char('y'), Key::Char('Y')], accept);

    h.add_prompt_handlers([Key::Char('n'), Key::Char('N'), Key::Enter], decline);

    h.add_prompt_handlers([Key::Esc, Key::Char('q'), Key::Ctrl('c')], cancel);
}

fn accept(data: &mut PromptData) -> Vec<LumaMessage> {
    if let Some(rx) = data.resp.take() {
        rx.send(PromptResponse::Yes).unwrap();
    }
    vec![LumaMessage::SetMode(Mode::Normal)]
}

fn decline(data: &mut PromptData) -> Vec<LumaMessage> {
    if let Some(rx) = data.resp.take() {
        rx.send(PromptResponse::No).unwrap();
    }
    vec![LumaMessage::SetMode(Mode::Normal)]
}

fn cancel(data: &mut PromptData) -> Vec<LumaMessage> {
    if let Some(rx) = data.resp.take() {
        rx.send(PromptResponse::Cancel).unwrap();
    }

    vec![LumaMessage::SetMode(Mode::Normal)]
}
