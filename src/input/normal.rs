use crate::{
    app::{Mode, State},
    event::Key,
    input::Msg,
};

pub fn handle(key: Key, stat: &mut State) -> Option<Msg> {
    let msg = match key {
        Key::Esc | Key::Char('q') | Key::Ctrl('c') => Msg::Quit,

        Key::Ins | Key::Char('e') => Msg::Edit,
        Key::Char('r') => Msg::RenameTab,

        Key::Char('a') => Msg::Add,
        Key::Char('n') => Msg::AddTab,

        Key::Enter | Key::Char('o') => Msg::Open,

        Key::Tab => Msg::SelectTab(stat.tabb + 1),
        Key::ShiftTab => Msg::SelectTab(stat.tabb.saturating_sub(1)),

        Key::Delete | Key::Backspace | Key::Char('d') => {
            Msg::ChangeMode(Mode::Delete(crate::app::Item::Link))
        }
        Key::Char('D') => Msg::ChangeMode(Mode::Delete(crate::app::Item::Tab)),

        Key::Up | Key::Char('k') => Msg::MoveUp(1),
        Key::Down | Key::Char('j') => Msg::MoveDown(1),

        Key::End | Key::Char('G') => Msg::MoveDown(usize::MAX),
        Key::Char('g') => Msg::MoveUp(usize::MAX),

        Key::PageUp | Key::Ctrl('u') => Msg::MoveUp(20),
        Key::PageDown | Key::Ctrl('d') => Msg::MoveDown(20),

        Key::Char('1') => Msg::SelectTab(0),
        Key::Char('2') => Msg::SelectTab(1),
        Key::Char('3') => Msg::SelectTab(2),
        Key::Char('4') => Msg::SelectTab(3),
        Key::Char('5') => Msg::SelectTab(4),
        Key::Char('6') => Msg::SelectTab(5),
        Key::Char('7') => Msg::SelectTab(6),
        Key::Char('8') => Msg::SelectTab(7),
        Key::Char('9') => Msg::SelectTab(8),
        Key::Char('0') => Msg::SelectTab(9),

        Key::Left | Key::Right => return None,
        Key::Home => return None,

        Key::Char(_) => return None,
        Key::Ctrl(_) => return None,
        Key::Alt(_) => return None,
        Key::F(_) => return None,
        Key::Unknown => return None,
    };
    Some(msg)
}

// use crate::event::Key;
// use crate::prelude::*;
// use crate::state::Link;
// use crate::state::data::{PromptData, PromptResponse};
//
// use super::Handler;
//
// pub fn add_all(h: &mut Handler) {
//     h.add_normal_handlers([Key::Char('q'), Key::Ctrl('c')], exit);
//
//     h.add_normal_handler(Key::Char('i'), go_insert);
//     h.add_normal_handler(Key::Char('e'), go_edit);
//
//     h.add_normal_handlers([Key::Down, Key::Char('j')], move_down);
//     h.add_normal_handlers([Key::Up, Key::Char('k')], move_up);
//
//     h.add_normal_handler(Key::Enter, select);
//
//     h.add_normal_handler(Key::Char('g'), go_top);
//     h.add_normal_handler(Key::Char('G'), go_bottom);
//
//     h.add_normal_handlers([Key::Char('D'), Key::Backspace], delete);
//
//     h.add_normal_handler(Key::Char('?'), show_help);
//
//     h.add_normal_handler(Key::Char('s'), save);
//
//     h.add_normal_handler(Key::Char('1'), numeric_key_option::<0>);
//     h.add_normal_handler(Key::Char('2'), numeric_key_option::<1>);
//     h.add_normal_handler(Key::Char('3'), numeric_key_option::<2>);
//     // h.add_normal_handler(Key::Char('4'), numeric_key_option::<3>);
// }
//
// fn exit() -> Vec<LumaMessage> {
//     vec![LumaMessage::Exit]
// }
//
// // It works (kinda)
// fn go_edit() -> Vec<LumaMessage> {
//     log::info!("starting edit process");
//     let (tab, index) = util::blocking_get_tab_and_index();
//
//     let text = {
//         let luma = util::globals::get_luma!();
//         let link = luma.get_index(tab).unwrap().1.get(index).unwrap();
//         yaml::to_string(link).unwrap()
//     };
//
//     let (tx, rx) = oneshot::channel::<String>();
//
//     let h = tokio::spawn(async move {
//         if let Ok(resp) = rx.await {
//             log::debug!("got data from editor");
//             if let Ok(link) = yaml::from_str(&resp) {
//                 let (tab, index) = util::get_tab_and_index().await;
//
//                 let mut luma = LUMA.write().await;
//                 let l = luma.get_index_mut(tab).unwrap().1.get_mut(index).unwrap();
//                 log::debug!("writing new data to thing");
//
//                 *l = link;
//             }
//         }
//         block_on(async { STATE.write().await }).unsaved_changes = true;
//         vec![LumaMessage::Redraw]
//     });
//
//     log::debug!("sending edtior open request");
//
//     vec![
//         LumaMessage::OpenEditor { text, resp: tx },
//         LumaMessage::AddHandle(("Finish Edit".into(), h)),
//     ]
// }
//
// fn go_insert() -> Vec<LumaMessage> {
//     log::info!("starting insert process");
//
//     let text = {
//         let link = Link::stub();
//         yaml::to_string(&link).unwrap()
//     };
//
//     let (tx, rx) = oneshot::channel::<String>();
//
//     let h = tokio::spawn(async move {
//         if let Ok(resp) = rx.await {
//             log::debug!("got data from editor");
//             if let Ok(link) = yaml::from_str(&resp) {
//                 let (tab, index) = util::get_tab_and_index().await;
//
//                 log::debug!("writing new data to thing");
//                 LUMA.write()
//                     .await
//                     .get_index_mut(tab)
//                     .unwrap()
//                     .1
//                     .insert(index, link);
//             }
//         }
//         block_on(async { STATE.write().await }).unsaved_changes = true;
//         vec![LumaMessage::Redraw]
//     });
//
//     log::debug!("sending edtior open request");
//
//     vec![
//         LumaMessage::OpenEditor { text, resp: tx },
//         LumaMessage::AddHandle(("Finish Insert".into(), h)),
//     ]
//     // let (tab, index) = block_on(util::get_tab_and_index());
//     //
//     // let (tx_link, rx_link) = oneshot::channel();
//     // let (tx_name, rx_name) = oneshot::channel();
//     //
//     // let _h = tokio::spawn(async move {
//     //     log::info!("insert request waiting on responses");
//     //     let name = rx_name.await.unwrap();
//     //     log::info!("got name: {}", name);
//     //     let link = rx_link.await.unwrap();
//     //     log::info!("got link: {}", link);
//     //
//     //     if let Some(set) = block_on(async { LUMA.write().await }).get_index_mut(tab) {
//     //         set.1.insert(index, Link::new(name, link))
//     //     }
//     // });
//     //
//     // vec![LumaMessage::SetMode(Mode::Insert(vec![
//     //     ("link: ".to_string(), String::new(), tx_link),
//     //     ("name: ".into(), String::new(), tx_name),
//     // ]))]
// }
//
// fn move_down() -> Vec<LumaMessage> {
//     log::trace!("Moving cursor down");
//     let mut state = block_on(async { STATE.write().await });
//
//     let max_len = block_on(async { LUMA.read().await })
//         .get_index(state.selected_tab)
//         .unwrap()
//         .1
//         .len();
//     if state.selected_index < max_len.saturating_sub(1) {
//         state.selected_index += 1;
//     }
//
//     vec![LumaMessage::Redraw]
// }
//
// fn move_up() -> Vec<LumaMessage> {
//     let mut state = block_on(async { STATE.write().await });
//
//     if state.selected_index > 0 {
//         state.selected_index -= 1;
//     }
//     vec![LumaMessage::Redraw]
// }
//
// fn save() -> Vec<LumaMessage> {
//     vec![LumaMessage::Save(None)]
// }
//
// fn select() -> Vec<LumaMessage> {
//     let (tab, index) = block_on(util::get_tab_and_index());
//     let h = {
//         let luma = block_on(async { LUMA.read().await });
//         let set = luma.get_index(tab).unwrap();
//         let link = set.1.get(index).unwrap();
//         log::info!("Opening link: {}", link.link);
//         // if let Some(path) = &link.file {
//         //     FILE_OPENER.open(path.as_str()).unwrap()
//         // } else {
//         LINK_OPENER.open(&link.link).unwrap()
//         // }
//     };
//     vec![LumaMessage::AddHandle(("Open Command".into(), h))]
// }
//
// fn go_top() -> Vec<LumaMessage> {
//     let mut state = block_on(async { STATE.write().await });
//     state.selected_index = 0;
//     vec![LumaMessage::Redraw]
// }
//
// fn go_bottom() -> Vec<LumaMessage> {
//     let mut state = block_on(async { STATE.write().await });
//     let max_index = block_on(async { LUMA.read().await })
//         .get_index(state.selected_tab)
//         .unwrap()
//         .1
//         .len();
//     state.selected_index = max_index - 1;
//     vec![LumaMessage::Redraw]
// }
//
// const HELP_TEXT: &str = "\
// j k     -       Move Up/Down\n\
// g G     -       Go Top/Bottom\n\
// Enter   -       Open Link in Browser\n\
// c-j c-k -       Move Link Up/Down\n\
// i       -       Add Link\n\
// D       -       Delete entry\n\
// q       -       Exit\
// ";
//
// fn show_help() -> Vec<LumaMessage> {
//     let (tx, rx) = oneshot::channel();
//
//     let h = tokio::spawn(async move {
//         rx.await.unwrap();
//         vec![LumaMessage::Redraw]
//     });
//
//     vec![
//         LumaMessage::SetMode(Mode::Prompt(PromptData {
//             prompt: HELP_TEXT.to_string().into_boxed_str(),
//             resp: Some(tx),
//         })),
//         LumaMessage::AddHandle(("Close Help".into(), h)),
//     ]
// }
//
// /// This currently works but is a bit hacky
// /// There might be some consideration to using a tokio::sync::RwLock so the write
// /// can be held over the await to garentee that no one changes it in the down time
// fn delete() -> Vec<LumaMessage> {
//     let (tab, index) = block_on(util::get_tab_and_index());
//
//     let name = block_on(async { LUMA.read().await })
//         .get_index(tab)
//         .unwrap()
//         .1
//         .get(index)
//         .unwrap()
//         .name
//         .clone();
//
//     let (tx, rx) = oneshot::channel();
//
//     let h = tokio::spawn(async move {
//         if let Ok(PromptResponse::Yes) = rx.await {
//             block_on(async { LUMA.write().await })
//                 .get_index_mut(tab)
//                 .unwrap()
//                 .1
//                 .remove(index);
//             vec![LumaMessage::Redraw]
//         } else {
//             vec![]
//         }
//     });
//
//     vec![
//         LumaMessage::SetMode(Mode::Prompt(PromptData {
//             prompt: format!("Remove audio \"{}\"? (y/N)", name).into_boxed_str(),
//             resp: Some(tx),
//         })),
//         LumaMessage::AddHandle(("Delete Link".into(), h)),
//     ]
// }
//
// //         Key::Left | Key::Char('h') => todo!(),
// //         Key::Right | Key::Char('l') => LumaMessage::Nothing,
//
// // fn handle_key_normal(key: Key, luma: &mut Luma) -> LumaMessage {
// //     match key {
// //
// //         Key::Ctrl('j') => {
// //             let index = match self.screen.get_selected_index() {
// //                 Some(i) => i,
// //                 None => return LumaMessage::Nothing,
// //             };
// //
// //             let tab = self.screen.get_selected_tab_index();
// //             let set = match luma.get_index_mut(tab) {
// //                 Some(kv) => kv.1,
// //                 None => return LumaMessage::Nothing,
// //             };
// //
// //             if index > 0 && index < set.len() - 1 {
// //                 set.swap(index, index + 1)
// //             }
// //             self.screen.move_cursor(1);
// //             LumaMessage::Redraw
// //         }
// //         Key::Ctrl('k') => {
// //             let index = match self.screen.get_selected_index() {
// //                 Some(i) => i,
// //                 None => return LumaMessage::Nothing,
// //             };
// //             let tab = self.screen.get_selected_tab_index();
// //             let set = luma.get_index_mut(tab).expect("Set must exists").1;
// //
// //             if index < set.len() - 1 {
// //                 set.swap(index, index - 1)
// //             }
// //
// //             // self.screen.move_up();
// //             LumaMessage::Redraw
// //         }
// //
// //
// //         // Key::Tab => self.screens.next_tab(),
// //         Key::Alt('e') => todo!("enter search"),
// //         Key::Char(_) => LumaMessage::Nothing,
// //         _ => todo!(),
// //     }
// // }
//
// fn numeric_key_option<const N: usize>() -> Vec<LumaMessage> {
//     let max_value = util::globals::get_luma!().len();
//     if N < max_value {
//         block_on(async { STATE.write().await }).selected_tab = N;
//     }
//     vec![LumaMessage::Redraw]
// }
