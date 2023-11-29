//! ## Model
//!
//! app model

use crate::prelude::*;

use super::components;
use super::screen::Screen;
use super::{Id, Msg, UserEvent};

use std::time::Duration;
use tuirealm::{Sub, SubClause, SubEventClause};
// use tuirealm::props::{Alignment, Color, TextModifiers};
use tuirealm::terminal::TerminalBridge;
// use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{Application, EventListenerCfg, Update};

pub struct Model {
    /// Application
    pub app: Application<Id, Msg, UserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: TerminalBridge,
    /// The configured size of the screen
    screen: Screen,
}

impl Model {
    pub fn new() -> Result<Self> {
        let mut app = Application::init(
            EventListenerCfg::default()
                // .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        let tabs: Vec<String> = {
            let luma = block_on(async { LUMA.read().await });
            let keys = luma.keys();
            keys.cloned().collect()
        };

        app.mount(Id::List, Box::new(components::ItemList::new()), vec![])?;
        app.mount(
            Id::GlobalListener,
            Box::new(components::GlobalListener::new()),
            vec![Sub::new(SubEventClause::Any, SubClause::Always)],
        )?;
        app.mount(
            Id::Tabs,
            Box::new(components::Tabbar::new(tabs.into_iter())),
            // TODO: give a more restrictive subscription
            vec![Sub::always()],
        )?;
        app.mount(
            Id::Preveiw,
            Box::new(components::Preveiw::new(String::from("test"))),
            vec![],
        )?;

        // Set active window
        app.active(&Id::GlobalListener).unwrap();

        let terminal = TerminalBridge::new()?;
        let tuirealm::tui::prelude::Rect { width, height, .. } = terminal.raw().size()?;

        let screen = Screen::new(width, height);

        Ok(Self {
            app,
            quit: false,
            redraw: true,
            terminal,
            screen,
        })
    }

    /// Renders the application to the screen
    pub fn view(&mut self) {
        self.terminal
            .raw_mut()
            .draw(|f| {
                let tuirealm::tui::prelude::Rect { width, height, .. } = f.size();
                self.screen.configure_surface(width, height);

                self.app.view(&Id::List, f, self.screen.tree_veiw);
                self.app.view(&Id::Tabs, f, self.screen.tab_bar);
                self.app.view(&Id::Preveiw, f, self.screen.preview);
            })
            .unwrap();
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        match msg {
            Some(Msg::AppClose) => {
                self.quit = true;
                log::info!("set close flag");
                None
            }
            Some(Msg::SetTab(i)) => {
                util::globals::get_state_mut!().selected_tab = i;
                // self.app
                //     .attr(
                //         &Id::List,
                //         tuirealm::Attribute::Custom("SetTab"),
                //         tuirealm::AttrValue::Number(i as isize),
                //     )
                //     .ok()?;
                None
            }
            Some(Msg::LinkChanged(_l)) => {
                // self.app
                //     .attr(
                //         &Id::Preveiw,
                //         tuirealm::Attribute::Text,
                //         tuirealm::AttrValue::String(l.text()),
                //     )
                //     .ok()?;
                None
            }
            // Some(_) => {
            //     self.redraw = true;
            //     None
            // }
            None => None,
        }
    }
}
