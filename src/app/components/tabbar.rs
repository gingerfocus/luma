use tuirealm::props::{Color, Style};
use tuirealm::tui::symbols;
use tuirealm::tui::text::Line;
use tuirealm::tui::widgets::Tabs;
use tuirealm::{Component, Event as REvent, MockComponent};

use crate::app::{Msg, UserEvent};
use crate::event::Key;
use crate::prelude::*;

pub struct Tabbar {
    state: TabState,
}

pub struct TabState {
    names: Vec<String>,
    selected: usize,
}

impl TabState {
    fn make_tabs<'a>(&'a self) -> Tabs<'a> {
        Tabs::new(
            self.names
                .iter()
                .cloned()
                .map(Line::from)
                .collect::<Vec<Line<'_>>>(),
        )
        .select(self.selected)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(symbols::DOT)
    }
}

impl Tabbar {
    pub fn new(names: impl Iterator<Item = String>) -> Self {
        // let state = TabState { selected: 0 };
        let state = TabState {
            names: names.collect(),
            selected: 0,
        };

        Self { state }
    }
}

impl MockComponent for Tabbar {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::prelude::Rect) {
        frame.render_widget(self.state.make_tabs(), area);
    }

    fn focus(&mut self, _gained: bool) {}
}

impl Component<Msg, UserEvent> for Tabbar {
    fn on(&mut self, ev: REvent<UserEvent>) -> Option<Msg> {
        log::info!("Got Event: {:?}", ev);
        match ev.into() {
            Event::Input(Key::Char('1')) => {
                self.state.selected = 0;
                Some(Msg::SetTab(0))
            }
            Event::Input(Key::Char('2')) => {
                self.state.selected = 1;
                Some(Msg::SetTab(1))
            }
            Event::Input(Key::Char('3')) => {
                self.state.selected = 2;
                Some(Msg::SetTab(2))
            }
            Event::Input(Key::Char('4')) => {
                self.state.selected = 3;
                Some(Msg::SetTab(3))
            }
            _ => None,
        }
    }
}
