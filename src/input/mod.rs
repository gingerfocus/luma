use crate::app::Mode;

pub mod delete;
pub mod normal;

#[derive(Debug)]
pub enum Msg {
    /// The app should quit
    Quit,
    /// The app should edit the selcted link
    Edit,
    /// Move down a number of lines
    MoveDown(usize),
    /// Move up a number of lines
    MoveUp(usize),
    /// Move to the give tab
    SelectTab(usize),
    /// Change to the given mode
    ChangeMode(Mode),
    /// The app should open the selected link
    Open,
    /// The app should delete the selected link
    Delete,
    /// Add a blank link then edit it
    Add,

    RenameTab,
    DeleteTab,
    AddTab,
}
