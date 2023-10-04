use crate::prelude::*;

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Normal,
    Prompt(PromptData),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptResponse {
    Yes,
    No,
    Cancel,
}

#[derive(Debug)]
pub struct PromptData {
    pub prompt: Box<str>,
    pub resp: Option<oneshot::Sender<PromptResponse>>,
}

/// First string is the prompt, second is the buffer that is written to
/// and the final is the sender that receives the buffer after it is submitted
pub type InsertData = Vec<(String, String, oneshot::Sender<String>)>;
