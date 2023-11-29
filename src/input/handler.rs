use std::collections::HashMap;

use crate::{event::Key, prelude::*, state::data::PromptData};

mod normal;
mod prompt;

type NormalFunction = fn() -> Vec<LumaMessage>;
type PromptFunction = fn(&mut PromptData) -> Vec<LumaMessage>;

#[derive(Default)]
pub struct Handler {
    normal_keys: HashMap<Key, NormalFunction>,
    prompt_keys: HashMap<Key, PromptFunction>,
}

impl Handler {
    pub fn new() -> Handler {
        Self::default()
    }

    pub fn add_normal_handler(&mut self, key: Key, f: NormalFunction) {
        self.normal_keys.insert(key, f);
    }
    pub fn add_normal_handlers(&mut self, keys: impl IntoIterator<Item = Key>, f: NormalFunction) {
        for key in keys {
            self.normal_keys.insert(key, f);
        }
    }

    pub fn add_prompt_handler(&mut self, key: Key, f: PromptFunction) {
        self.prompt_keys.insert(key, f);
    }
    pub fn add_prompt_handlers(&mut self, keys: impl IntoIterator<Item = Key>, f: PromptFunction) {
        for key in keys {
            self.add_prompt_handler(key, f)
        }
    }

    pub async fn handle(&mut self, key: Key, mode: &mut Mode) -> Vec<LumaMessage> {
        match mode {
            Mode::Normal => match self.normal_keys.get(&key) {
                Some(f) => f(),
                None => vec![],
            },
            Mode::Prompt(data) => match self.prompt_keys.get(&key) {
                Some(f) => f(data),
                None => vec![],
            },
        }
    }
}
