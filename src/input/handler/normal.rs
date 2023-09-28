use crate::prelude::*;

// macro_rules! numeric_key_option {
//     ( $self:ident, $luma:ident, $x:expr ) => {{
//         let name: String = match $luma.get_index($x) {
//             Some(n) => n.0.clone(),
//             None => return LumaMessage::Nothing,
//         };
//
//         let ind = $crate::ui::screen::IndexedPair { index: $x, name };
//         $self.screen.select_tab(ind);
//         LumaMessage::Redraw
//     }};
// }

use crate::input::Key;

use super::Handler;

pub fn add_all(h: &mut Handler) {
    h.add_normal_handlers([Key::Char('q'), Key::Ctrl('c')], exit);

    // h.add_normal_handler(Key::Char('i'), go_insert);
    // h.add_normal_handler(Key::Char('e'), go_edit);

    h.add_normal_handlers([Key::Down, Key::Char('j')], move_down);
    h.add_normal_handlers([Key::Up, Key::Char('k')], move_up);

    h.add_normal_handler(Key::Enter, select);

    h.add_normal_handler(Key::Char('g'), go_top);
    h.add_normal_handler(Key::Char('G'), go_bottom);

    // h.add_normal_handlers([Key::Char('D'), Key::Backspace], delete);

    // h.add_normal_handler(Key::Char('?'), show_help);
}

fn exit() -> Option<LumaMessage> {
    LumaMessage::Exit.into()
}

// fn go_edit() -> Option<LumaMessage> {
//     let read = STATE.read().unwrap();
//     let tab = read.selected_tab;
//     let index = read.selected_index;
//     drop(read);
//
//     let luma = LUMA.read().unwrap();
//     let link = luma.get_index(tab).unwrap().1.get(index)?;
//
//     let default = format!(
//         "Name: {}\nLink: {}\nFile: {}\nArtist:{}\n",
//         &link.name,
//         &link.link,
//         link.file.as_deref().unwrap_or_default(),
//         link.artist.as_deref().unwrap_or_default()
//     );
//     drop(luma);
//
//     let (tx, rx) = oneshot::channel::<String>();
//
//     tokio::spawn(async move {
//         if let Ok(resp) = rx.await {
//             let mut res: Vec<&str> = resp.lines().map(|l| l.split_once(':').unwrap().1).collect();
//             let file = res.pop().unwrap(); // index 3
//             let artist = res.pop().unwrap(); // index 2
//             let name = res.pop().unwrap(); // index 1
//             let link = res.pop().unwrap(); // index 0
//
//             let (tab, index) = {
//                 let gread = STATE.read().unwrap();
//                 (gread.selected_tab, gread.selected_index)
//             };
//
//             let mut wluma = LUMA.write().unwrap();
//             let l = wluma.get_index_mut(tab).unwrap().1.get_mut(index).unwrap();
//
//             l.name = name.to_string();
//             l.link = link.to_string();
//             l.file = file.try_into().ok();
//             l.artist = artist.try_into().ok();
//         }
//     });
//
//     LumaMessage::AskQuestion(
//         crate::QuestionComponents {
//             name: "Edit Link".into(),
//             _question: crate::QuestionType::Editor,
//             default,
//         },
//         tx,
//     )
//     .into()
//     // Some(LumaMessage::AskQuestion(q, &(edit_link as fn(Answer))))
// }

// fn go_insert() -> Option<LumaMessage> {
//     let read = STATE.read().unwrap();
//     let tab = read.selected_tab;
//     let index = read.selected_index;
//     drop(read);
//
//     let callback = Rc::new(move |luma: &mut Luma, mut buffers: Vec<String>| {
//         let name = buffers.pop().unwrap(); // index 1
//         let link = buffers.pop().unwrap(); // index 0
//
//         luma.get_index_mut(tab)
//             .unwrap()
//             .1
//             .insert(index, Link::new(name, link));
//     });
//
//     // let prompts = ["link".into(), "name".into(), "".into(), "".into()];
//
//     let (tx_link, rx_link) = tokio::sync::oneshot::channel();
//     let (tx_name, rx_name) = tokio::sync::oneshot::channel();
//
//     LumaMessage::SetMode(Mode::Insert(vec![
//         ("link: ".to_string(), String::new(), tx_link),
//         ("name: ".into(), String::new(), tx_name),
//     ]))
//     .into()
// }

fn move_down() -> Option<LumaMessage> {
    log::debug!("Moving cursor down");
    let mut gwrite = STATE.write().unwrap();

    let max_len = LUMA
        .read()
        .unwrap()
        .get_index(gwrite.selected_tab)
        .unwrap()
        .1
        .len();
    if gwrite.selected_index < max_len - 1 {
        gwrite.selected_index += 1;
    }

    LumaMessage::Redraw.into()
}

fn move_up() -> Option<LumaMessage> {
    let mut gwrite = STATE.write().unwrap();

    if gwrite.selected_index > 0 {
        gwrite.selected_index -= 1;
    }
    LumaMessage::Redraw.into()
}

fn select() -> Option<LumaMessage> {
    let gread = STATE.read().unwrap();
    let tab = gread.selected_tab;
    let index = gread.selected_index;
    drop(gread);

    let luma = LUMA.read().unwrap();
    let set = luma.get_index(tab).unwrap();
    let link = set.1.get(index).unwrap();
    log::info!("Opening link: {}", link.link);
    AUDIO_OPENER.open(&link.link);
    None
}

fn go_top() -> Option<LumaMessage> {
    let mut state = STATE.write().unwrap();
    state.selected_index = 0;
    LumaMessage::Redraw.into()
}

fn go_bottom() -> Option<LumaMessage> {
    let mut state = STATE.write().unwrap();
    let max_index = LUMA
        .read()
        .unwrap()
        .get_index(state.selected_tab)
        .unwrap()
        .1
        .len();
    state.selected_index = max_index - 1;
    LumaMessage::Redraw.into()
}

// const HELP_TEXT: &str = "\
// j k     -       Move Up/Down\n\
// g G     -       Go Top/Bottom\n\
// Enter   -       Open Link in Browser\n\
// c-j c-k -       Move Link Up/Down\n\
// i       -       Add Link\n\
// D       -       Delete entry\n\
// q       -       Exit\
// ";

// fn show_help() -> Option<LumaMessage> {
//     let h = tokio::spawn(async move {
//         let (tx, rx) = oneshot::channel();
//
//         // *MODE.write().unwrap() = Mode::Prompt(crate::mode::PromptData {
//         //     prompt: HELP_TEXT.to_string().into_boxed_str(),
//         //     resp: Some(tx),
//         // });
//         _ = rx.await;
//         // *MODE.write().unwrap() = Mode::Normal;
//     });
// }

// fn delete() {
//     let h = tokio::spawn(async move {
//         let (tab, index) = {
//             let state = STATE.read().unwrap();
//             (state.selected_tab, state.selected_index)
//         };
//
//         let name = LUMA
//             .read()
//             .unwrap()
//             .get_index(tab)
//             .unwrap()
//             .1
//             .get(index)
//             .unwrap()
//             .name
//             .clone();
//
//         let (tx, rx) = oneshot::channel();
//
//         *MODE.write().unwrap() = Mode::Prompt(crate::mode::PromptData {
//             prompt: format!("Remove audio \"{}\"? (y/N)", name).into_boxed_str(),
//             resp: Some(tx),
//         });
//
//         if let Ok(PromptResponse::Yes) = rx.await {
//             LUMA.write()
//                 .unwrap()
//                 .get_index_mut(tab)
//                 .expect("thing")
//                 .1
//                 .remove(index);
//         }
//     });
// }

//         Key::Left | Key::Char('h') => todo!(),
//         Key::Right | Key::Char('l') => LumaMessage::Nothing,

// fn handle_key_normal(key: Key, luma: &mut Luma) -> LumaMessage {
//     match key {
//
//         Key::Ctrl('j') => {
//             let index = match self.screen.get_selected_index() {
//                 Some(i) => i,
//                 None => return LumaMessage::Nothing,
//             };
//
//             let tab = self.screen.get_selected_tab_index();
//             let set = match luma.get_index_mut(tab) {
//                 Some(kv) => kv.1,
//                 None => return LumaMessage::Nothing,
//             };
//
//             if index > 0 && index < set.len() - 1 {
//                 set.swap(index, index + 1)
//             }
//             self.screen.move_cursor(1);
//             LumaMessage::Redraw
//         }
//         Key::Ctrl('k') => {
//             let index = match self.screen.get_selected_index() {
//                 Some(i) => i,
//                 None => return LumaMessage::Nothing,
//             };
//             let tab = self.screen.get_selected_tab_index();
//             let set = luma.get_index_mut(tab).expect("Set must exists").1;
//
//             if index < set.len() - 1 {
//                 set.swap(index, index - 1)
//             }
//
//             // self.screen.move_up();
//             LumaMessage::Redraw
//         }
//
//         Key::Char('1') => numeric_key_option!(self, luma, 0),
//         Key::Char('2') => numeric_key_option!(self, luma, 1),
//         Key::Char('3') => numeric_key_option!(self, luma, 2),
//         Key::Char('4') => numeric_key_option!(self, luma, 3),
//
//         // Key::Tab => self.screens.next_tab(),
//         Key::Alt('e') => todo!("enter search"),
//         Key::Char(_) => LumaMessage::Nothing,
//         _ => todo!(),
//     }
// }
