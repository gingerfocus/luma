// type ChosenFunction = dyn Fn(&mut Luma);
// type ReadStringsFunction = dyn Fn(&mut Luma, Vec<String>);

#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    Prompt(PromptData),
    // Insert(InsertData),
    // Visual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptResponse {
    Yes,
    No,
    Cancel,
}

pub struct PromptData {
    pub prompt: Box<str>,
    pub resp: Option<tokio::sync::oneshot::Sender<PromptResponse>>,
    // pub accepted: Rc<ChosenFunction>,
    // pub declined: Rc<ChosenFunction>,
}

/// First string is the prompt, second is the buffer that is written to
/// and the final is the sender that receives the buffer after it is submitted
pub type InsertData = Vec<(String, String, tokio::sync::oneshot::Sender<String>)>;
// pub callback: Rc<ReadStringsFunction>,

// impl InsertData {
// pub fn new(prompts: [String; 4], callback: Rc<ReadStringsFunction>) -> Result<Self> {
//     Self::new_with_buffers(
//         prompts,
//         [String::new(), String::new(), String::new(), String::new()],
//         callback,
//     )
// }

// pub fn new_with_buffers(
//     prompts: [String; 4],
//     buffers: [String; 4],
//     callback: Rc<ReadStringsFunction>,
// ) -> Result<Self> {
//     let num_prompts = prompts.iter().filter(|p| !p.is_empty()).count();
//     Ok(Self {
//         index: 0,
//         num_prompts,
//         prompts,
//         buffers,
//         callback,
//     })
// }

// A method (that is only used once) that moves to the next index and
// returns None. If it is the last index it instead ruturns the buffers and
// the function to call withing the stuct
//     pub fn next_or_destructure(&mut self) -> Option<(Vec<String>, Rc<ReadStringsFunction>)> {
//         // subtract 1 beacuse index is 0 indexed and prompts is 1 indexed
//         if self.index < self.num_prompts - 1 {
//             self.index += 1;
//             None
//         } else {
//             let buffers = self
//                 .buffers
//                 .iter()
//                 .take(self.num_prompts)
//                 .cloned()
//                 .collect();
//             Some((buffers, self.callback.clone()))
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use crate::prelude::*;
//     use std::rc::Rc;
//
//     #[test]
//     fn check_insert_data_destructure() {
//         let callback = Rc::new(|_luma: &mut Luma, _buffers: Vec<String>| {});
//         let prompts = ["test".into(), "more".into(), "thing".into(), "".into()];
//         let data = super::InsertData::new(prompts, callback);
//
//         assert!(data.is_ok());
//         let mut data = data.unwrap();
//
//         // Only as there are 3 items only two nexts should be needed as it
//         // starts on the first one
//         assert!(data.next_or_destructure().is_none());
//         assert!(data.next_or_destructure().is_none());
//         assert!(data.next_or_destructure().is_some());
//     }
//
//     #[test]
//     fn test_return_buffer_validity() {
//         let callback = Rc::new(|_luma: &mut Luma, buffers: Vec<String>| {
//             let mut buffers = buffers;
//             assert!(buffers.len() == 2);
//             // we did nothing to the data so empty response is expected
//             assert!(buffers.pop() == Some("".into()));
//             buffers.pop();
//             assert!(buffers.pop() == None);
//         });
//
//         let prompts = ["first".into(), "second".into(), "".into(), "".into()];
//         let data = super::InsertData::new(prompts, callback);
//
//         assert!(data.is_ok());
//         let mut data = data.unwrap();
//
//         assert!(data.next_or_destructure().is_none());
//
//         let Some((buffers, callback)) = data.next_or_destructure() else {
//             assert!(false);
//
//             // this should never run
//             loop {}
//         };
//
//         let mut luma = Luma::default();
//         callback(&mut luma, buffers);
//     }
// }
