use tuirealm::{Component, Event as REvent, MockComponent};

use crate::app::{Msg, UserEvent};
use crate::event::Key;
use crate::prelude::*;

pub struct GlobalListener {}

impl GlobalListener {
    pub fn new() -> Self {
        Self {}
    }
}

impl MockComponent for GlobalListener {
    fn view(&mut self, _frame: &mut tuirealm::Frame, _area: tuirealm::tui::prelude::Rect) {}

    fn focus(&mut self, _gained: bool) {}
}

impl Component<Msg, UserEvent> for GlobalListener {
    fn on(&mut self, event: REvent<UserEvent>) -> Option<Msg> {
        log::trace!("Got Event: {:?}", event);
        match event.into() {
            Event::Input(Key::Esc) => Some(Msg::AppClose),
            Event::Input(Key::Char('q')) => Some(Msg::AppClose),
            Event::Input(Key::Ctrl('c')) => Some(Msg::AppClose),
            _ => None,
        }
    }
}
