use std::collections::HashMap;

use tokio::time::Instant;

use crate::{
    mode::{InsertData, PromptData},
    prelude::*,
};

mod insert;
mod normal;
mod prompt;

use super::key::Key;

type NormalFunction = fn() -> Vec<LumaMessage>;
type InsertFunction = fn(&mut InsertData) -> Vec<LumaMessage>;
type PromptFunction = fn(&mut PromptData) -> Vec<LumaMessage>;

#[derive(Default)]
pub struct Handler {
    normal_keys: HashMap<Key, NormalFunction>,
    /// Bindings willl call their call back when they exist, unlike the other
    /// two when this binding is missing it will write the character to the
    /// current buffer.
    insert_keys: HashMap<Key, InsertFunction>,
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
        insert::add_all(self);
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

    pub fn add_insert_handler(&mut self, key: Key, f: InsertFunction) {
        self.insert_keys.insert(key, f);
    }
    pub fn add_insert_handlers(&mut self, keys: impl IntoIterator<Item = Key>, f: InsertFunction) {
        for key in keys {
            self.insert_keys.insert(key, f);
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

    pub fn handle(&mut self, key: Key, mode: &GlobalMode) -> Vec<LumaMessage> {
        match &mut mode.write().unwrap() as &mut Mode {
            Mode::Normal => match self.normal_keys.get(&key) {
                Some(f) => f(),
                None => vec![],
            },
            Mode::Insert(data) => match self.insert_keys.get(&key) {
                Some(f) => f(data),
                None => write_char_to_insert_data(key, data),
            },
            Mode::Prompt(data) => match self.prompt_keys.get(&key) {
                Some(f) => f(data),
                None => vec![],
            },
        }
    }
}

fn write_char_to_insert_data(k: Key, data: &mut InsertData) -> Vec<LumaMessage> {
    if let Key::Char(c) = k {
        data.last_mut().unwrap().1.push(c);
        vec![LumaMessage::Redraw]
    } else {
        vec![]
    }
}
