pub mod task;


mod key;
mod mouse;
use crate::app::UserEvent;

pub use self::{key::Key, mouse::{Mouse, MouseKind}};

use crossterm::event::Event as CrossEvent;
use tuirealm::NoUserEvent;

/// An occurred event.
#[derive(Default, Debug)]
pub enum Event {
    /// An input event occurred.
    Input(Key),
    /// When the mouse is clicked
    Click(Mouse),
    /// when the terminal is resized
    Resize(u16, u16),
    /// A change in focus for the screen, true when gained, false when lost
    GainedFocus(bool),
    /// An tick event occurred.
    #[default]
    Tick,
    /// Repersents a lack of an event
    Nothing,
    /// Repersents a user/plugin made event
    User(UserEvent)
}

impl From<CrossEvent> for Event {
    fn from(value: crossterm::event::Event) -> Self {
        match value {
            CrossEvent::FocusGained => Event::GainedFocus(true),
            CrossEvent::FocusLost => Event::GainedFocus(false),
            CrossEvent::Key(ke) => Event::Input(ke.into()),
            CrossEvent::Mouse(me) => Event::Click(me.into()),
            CrossEvent::Paste(_) => unimplemented!(),
            CrossEvent::Resize(x, y) => Event::Resize(x, y),
        }
    }
}

impl From<tuirealm::Event<UserEvent>> for Event {
    fn from(value: tuirealm::Event<UserEvent>) -> Self {
        match value {
            tuirealm::Event::FocusGained => Event::GainedFocus(true),
            tuirealm::Event::FocusLost => Event::GainedFocus(false),
            tuirealm::Event::Keyboard(ke) => Event::Input(ke.into()),
            tuirealm::Event::Paste(_) => unimplemented!(),
            tuirealm::Event::WindowResize(x, y) => Event::Resize(x, y),
            tuirealm::Event::Tick => Event::Tick,
            tuirealm::Event::None => Event::Nothing,
            tuirealm::Event::User(e) => Event::User(e)
        }
    }
}
