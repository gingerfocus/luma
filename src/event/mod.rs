mod key;
mod mouse;

pub use self::{key::Key, mouse::Mouse};

/// An occurred event.
#[derive(Debug)]
pub enum Event {
    /// An input event occurred.
    Input(Key),
    /// When the mouse is clicked
    Click(Mouse),
    /// when the terminal is resized
    Resize(u16, u16),
    /// A change in focus for the screen, true when gained, false when lost
    GainedFocus(bool),
}

impl From<crossterm::event::Event> for Event {
    fn from(value: crossterm::event::Event) -> Self {
        use crossterm::event::Event as E;
        match value {
            E::FocusGained => Event::GainedFocus(true),
            E::FocusLost => Event::GainedFocus(false),
            E::Key(ke) => Event::Input(ke.into()),
            E::Mouse(me) => Event::Click(me.into()),
            E::Paste(_) => unimplemented!(),
            E::Resize(x, y) => Event::Resize(x, y),
        }
    }
}
