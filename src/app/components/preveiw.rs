use tuirealm::{tui::widgets::Paragraph, Component, Event as REvent, MockComponent};

// use tui_realm_stdlib::Paragraph;

use crate::{
    app::{Msg, UserEvent},
    event::Event,
};

#[derive(Default)]
pub struct Preveiw {
    // component: Paragraph,
    text: String,
}

impl MockComponent for Preveiw {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::prelude::Rect) {
        let widget = Paragraph::new(self.text.as_str());
        frame.render_widget(widget, area)
    }

    fn focus(&mut self, _gained: bool) {
        // prep to suspend or something or open editor in background
    }
}

impl Preveiw {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl Component<Msg, UserEvent> for Preveiw {
    fn on(&mut self, ev: REvent<UserEvent>) -> Option<Msg> {
        match ev.into() {
            Event::Input(crate::event::Key::Enter) => {
                // move editor to fg
                todo!()
            }
            // Event::User(UserEvent::)
            _ => None,
        }
    }
}
