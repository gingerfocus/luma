use crate::prelude::*;
use crate::{input::Key, mode::PromptData, LumaMessage};

use super::Handler;

pub fn add_all(h: &mut Handler) {
    h.add_prompt_handlers([Key::Char('y'), Key::Char('Y')], accept);

    h.add_prompt_handlers([Key::Char('n'), Key::Char('N'), Key::Enter], decline);

    h.add_prompt_handlers([Key::Esc, Key::Char('q'), Key::Ctrl('c')], cancel);
}

fn accept(state: &mut Luma, pd: &mut PromptData) -> Option<LumaMessage> {
    (pd.accepted)(state);
    Some(LumaMessage::SetMode(Box::new(Mode::Normal)))
}

fn decline(state: &mut Luma, pd: &mut PromptData) -> Option<LumaMessage> {
    (pd.declined)(state);
    Some(LumaMessage::SetMode(Box::new(Mode::Normal)))
}

fn cancel(_state: &mut Luma, _pd: &mut PromptData) -> Option<LumaMessage> {
    Some(LumaMessage::SetMode(Box::new(Mode::Normal)))
}
