use super::data::PromptData;

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Normal,
    Prompt(PromptData),
}

