use std::collections::HashMap;

use crate::{
    mode::{InsertData, PromptData},
    prelude::*,
};

mod insert;
mod normal;
mod prompt;

use super::key::Key;

type NormalFunction = fn(&mut Luma) -> Option<LumaMessage>;
type InsertFunction = fn(&mut Luma, &mut InsertData) -> Option<LumaMessage>;
type PromptFunction = fn(&mut Luma, &mut PromptData) -> Option<LumaMessage>;

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
        Self {
            normal_keys: Default::default(),
            insert_keys: Default::default(),
            prompt_keys: Default::default(),
        }
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

    pub fn handle(&self, key: Key, state: &mut Luma, mode: &mut Mode) -> Option<LumaMessage> {
        match mode {
            Mode::Normal => self.normal_keys.get(&key).and_then(|f| f(state)),
            Mode::Insert(data) => match self.insert_keys.get(&key) {
                Some(f) => f(state, data),
                None => write_char_to_insert_data(key, data),
            },
            Mode::Prompt(pd) => self.prompt_keys.get(&key).and_then(|f| f(state, pd)),
        }
    }
}

fn write_char_to_insert_data(k: Key, data: &mut InsertData) -> Option<LumaMessage> {
    if let Key::Ctrl(c) = k {
        data.buffers.get_mut(data.index)?.push(c);
        Some(LumaMessage::Redraw)
    } else {
        None
    }
}
