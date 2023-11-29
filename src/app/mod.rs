use crate::state::{GlobalSet, GlobalLink};

mod components;
mod screen;
pub mod model;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Msg {
    AppClose,
    TabChanged(GlobalSet),
    LinkChanged(GlobalLink),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    List,
    Preveiw,
    Tabs,
    GlobalListener,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, PartialOrd)]
pub enum UserEvent {
    LinkChanged(GlobalLink),
}

// use crate::prelude::*;

// pub struct App {
//     pub run: bool,
//     terminal: Terminal<CrosstermBackend<io::Stdout>>,
// }
//
// impl App {
//     pub fn new() -> Result<Self> {
//         let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
//
//         Ok(Self {
//             run: true,
//             screen: Default::default(),
//             terminal,
//         })
//     }
//
//     pub fn redraw(&mut self, mode: &Mode) -> Result<()> {
//         self.terminal.draw(|f| f.render_widget(Clear, f.size()))?;
//         self.draw(mode)
//     }
//
//     pub fn draw(&mut self, mode: &Mode) -> Result<()> {
//         log::debug!("redraw started");
//         self.terminal.draw(|f| {
//             // configure the surface so out drawing boxes are the right size
//             // if they changed since last render.
//             let Rect { width, height, .. } = f.size();
//             self.screen.configure_surface(width, height);
//
//             let (tab, index) = futures::executor::block_on(util::get_tab_and_index());
//             log::trace!("selected tab is: {}", tab);
//             log::trace!("selected index is: {}", index);
//
//             {
//                 // let luma = block_on(async { LUMA.read().await });
//                 //
//                 // let set = luma.get_index(tab).expect("A valid tab is not selected");
//                 //
//                 // if let Some(link) = set.1.get(index) {
//                 //     // TODO: when deleting the last item the cursor hovers nothing
//                 //
//                 //     let preview = link.as_paragraph();
//                 //     f.render_widget(preview, self.screen.preview_pane);
//                 // }
//                 //
//                 // let list = crate::ui::render::list(set.1);
//                 //
//                 // let mut state = ListState::default();
//                 // state.select(Some(index));
//                 // f.render_stateful_widget(list, self.screen.side_pane, &mut state);
//                 //
//                 // let names = luma.keys();
//                 // let tabs = crate::ui::render::tabs(names, tab);
//                 // f.render_widget(tabs, self.screen.title_bar);
//             }
//
//             match mode {
//                 Mode::Normal => {}
//                 Mode::Prompt(_data) => {
//                     // let prompt_render = crate::ui::render::prompt(&data.prompt);
//                     // let float_box = crate::ui::render::float_box(&data.prompt, width, height);
//
//                     // f.render_widget(Clear, float_box);
//                     // f.render_widget(prompt_render, float_box);
//                 }
//             }
//         })?;
//
//         log::debug!("redraw done");
//
//         Ok(())
//     }
// }
//
// impl Drop for App {
//     fn drop(&mut self) {
//         self.deinit().unwrap();
//     }
// }
