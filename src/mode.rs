use anyhow::Ok;

use crate::prelude::*;
use std::rc::Rc;

type ChosenFunction = dyn Fn(&mut Luma);
type ReadStringsFunction = dyn Fn(&mut Luma, Vec<String>);

#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    Prompt(PromptData),
    Insert(InsertData),
    // Visual,
}

pub struct PromptData {
    pub prompt: Box<str>,
    pub accepted: Rc<ChosenFunction>,
    pub declined: Rc<ChosenFunction>,
}

pub struct InsertData {
    pub index: usize,
    pub num_prompts: usize,
    pub prompts: [Box<str>; 4],
    pub buffers: [String; 4],
    pub callback: Rc<ReadStringsFunction>,
}

impl InsertData {
    pub fn new(prompts: [Box<str>; 4], callback: Rc<ReadStringsFunction>) -> anyhow::Result<Self> {
        let num_prompts = prompts
            .iter()
            .enumerate()
            .find(|(_i, p)| p.is_empty())
            .map(|(i, _p)| i)
            .unwrap_or(prompts.len());

        let buffers = [String::new(), String::new(), String::new(), String::new()];
        Ok(Self {
            index: 0,
            num_prompts,
            prompts,
            buffers,
            callback,
        })
    }

    /// A method (that is only used once) that moves to the next index and
    /// returns None. If it is the last index it instead ruturns the buffers and
    /// the function to call withing the stuct
    pub fn next_or_destructure(&mut self) -> Option<(Vec<String>, Rc<ReadStringsFunction>)> {
        // subtract 1 beacuse index is 0 indexed and prompts is 1 indexed
        if self.index < self.num_prompts - 1 {
            self.index += 1;
            None
        } else {
            let buffers = self
                .buffers
                .iter()
                .take(self.num_prompts)
                .cloned()
                .collect();
            Some((buffers, self.callback.clone()))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use std::rc::Rc;

    #[test]
    fn check_insert_data_destructure() {
        let callback = Rc::new(|_luma: &mut Luma, _buffers: Vec<String>| {});
        let prompts = ["test".into(), "more".into(), "thing".into(), "".into()];
        let data = super::InsertData::new(prompts, callback);

        assert!(data.is_ok());
        let mut data = data.unwrap();

        // Only as there are 3 items only two nexts should be needed as it
        // starts on the first one
        assert!(data.next_or_destructure().is_none());
        assert!(data.next_or_destructure().is_none());
        assert!(data.next_or_destructure().is_some());
    }

    #[test]
    fn test_return_buffer_validity() {
        let callback = Rc::new(|_luma: &mut Luma, buffers: Vec<String>| {
            assert!(buffers.len() == 2);
            assert!(buffers.get(3) == None);
        });

        let prompts = ["first".into(), "second".into(), "".into(), "".into()];
        let data = super::InsertData::new(prompts, callback);

        assert!(data.is_ok());
        let mut data = data.unwrap();

        dbg!(data.num_prompts);
        assert!(data.next_or_destructure().is_none());

        if let Some((buffers, callback)) = data.next_or_destructure() {
            let mut luma = Luma::default();
            callback(&mut luma, buffers);
        } else {
            assert!(false);
        }
    }
}
