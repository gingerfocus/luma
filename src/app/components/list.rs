use tuirealm::{
    props::{BorderSides, Color, Style},
    tui::widgets::{Block, List, ListItem, ListState},
    Component, MockComponent,
};

use crate::{
    app::{Msg, UserEvent},
    ui::Render,
};
use crate::{event::Key, prelude::*};

pub struct ItemList {
    // component: TreeView,
    state: ItemListState,
}

impl ItemList {
    pub fn new() -> Self {
        let state = ItemListState { selected: 0 };

        Self { state }
    }
}

impl MockComponent for ItemList {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::prelude::Rect) {
        let things = {
            let (tab, _) = util::blocking_get_tab_and_index();

            let luma = block_on(async { LUMA.read().await });
            let set = luma.get_index(tab).unwrap();

            set.1.iter().map(Render::render).collect::<Vec<ListItem>>()
        };

        let list = List::new(things)
            // .highlight_symbol(">")
            .highlight_style(Style::default().bg(Color::Red))
            .block(Block::default().title("links").borders(BorderSides::ALL));

        // frame.render_widget(list, area);
        let mut a = ListState::default();
        a.select(Some(self.state.selected));

        frame.render_stateful_widget(list, area, &mut a);
    }

    fn focus(&mut self, _gained: bool) {}

    // fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
    //     match (attr, value) {
    //         (tuirealm::Attribute::Focus, tuirealm::AttrValue::Flag(_)) => {}
    //         (tuirealm::Attribute::Custom("SetTab"), tuirealm::AttrValue::Number(_index)) => {
    //             todo!()
    //         }
    //         (a, v) => {
    //             log::warn!("Bad attribute request. Can't assign ({:?}) to ({:?})", a, v);
    //         }
    //     }
    // }
}

impl Component<Msg, UserEvent> for ItemList {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<Msg> {
        match ev.into() {
            // Event::Input(Key::Char('h')) => self.perform(Cmd::Custom(TREE_CMD_CLOSE)),
            // Event::Input(Key::Char('l')) => self.perform(Cmd::Custom(TREE_CMD_OPEN)),
            Event::Input(Key::Char('j')) => {
                util::globals::get_state_mut!().selected_index += 1;
                Some(Msg::LinkChanged(todo!()))
            }
            Event::Input(Key::Char('k')) => {
                util::globals::get_state_mut!().selected_index -= 1;
                None
            }
            Event::Input(Key::Char('g')) => {
                util::globals::get_state_mut!().selected_index = 0;
                None
            }
            // Event::Input(Key::Char('G')) => self.perform(Cmd::GoTo(Position::End)),
            // Event::Input(Key::Enter) => self.perform(Cmd::Submit),
            Event::User(UserEvent::LinkChanged(l)) => todo!(),
            _ => None,
        }

        // match result {
        //     CmdResult::Changed(s) => {
        //         log::info!("Got new state: {:?}", s);
        //         None
        //     }
        //     CmdResult::Submit(_res) => todo!(),
        //     CmdResult::Custom(_msg) => todo!(),
        //     CmdResult::Batch(_results) => todo!(),
        //     CmdResult::None => None,
        //     CmdResult::Invalid(cmd) => {
        //         log::warn!("Invalid Command for treeveiw: {:?}", cmd);
        //         None
        //     }
        // }
    }
}

pub struct ItemListState {
    selected: usize,
}
