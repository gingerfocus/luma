use crate::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    Prompt {
        msg: String,
        callback: Rc<dyn Fn(&mut Luma)>,
    },
    Insert {
        index: Rc<RefCell<usize>>,
        prompts: Rc<Vec<String>>,
        buffers: Rc<RefCell<Vec<String>>>,
        #[allow(clippy::type_complexity)]
        callback: Rc<dyn Fn(&mut Luma, Vec<String>)>,
    },
    // Visual,
}
