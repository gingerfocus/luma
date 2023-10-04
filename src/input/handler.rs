use std::collections::HashMap;

use tokio::time::Instant;

use crate::{event::key::Key, prelude::*, state::mode::PromptData};

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
        let mut s = Self::default();

        let time_i = Instant::now();
        s.add_default_binds();
        log::info!("Adding key binds took {:?}", time_i.elapsed());
        s
    }

    pub fn add_default_binds(&mut self) {
        normal::add_all(self);
        prompt::add_all(self);
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

    pub async fn handle(&mut self, key: Key, mode: &GlobalMode) -> Vec<LumaMessage> {
        match &mut mode.write().await as &mut Mode {
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
