use futures::executor::block_on;
use tokio::sync::oneshot;

use crate::mode::PromptResponse;
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
use crate::state::Link;

use super::Handler;

pub fn add_all(h: &mut Handler) {
    h.add_normal_handlers([Key::Char('q'), Key::Ctrl('c')], exit);

    h.add_normal_handler(Key::Char('i'), go_insert);
    // h.add_normal_handler(Key::Char('e'), go_edit);

    h.add_normal_handlers([Key::Down, Key::Char('j')], move_down);
    h.add_normal_handlers([Key::Up, Key::Char('k')], move_up);

    h.add_normal_handler(Key::Enter, select);

    h.add_normal_handler(Key::Char('g'), go_top);
    h.add_normal_handler(Key::Char('G'), go_bottom);

    h.add_normal_handlers([Key::Char('D'), Key::Backspace], delete);

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

fn go_insert() -> Option<LumaMessage> {
    let (tab, index) = block_on(util::get_tab_and_index());

    let (tx_link, rx_link) = oneshot::channel();
    let (tx_name, rx_name) = oneshot::channel();

    let _h = tokio::spawn(async move {
        log::info!("insert request waiting on responses");
        let name = rx_name.await.unwrap();
        log::info!("got name: {}", name);
        let link = rx_link.await.unwrap();
        log::info!("got link: {}", link);

        if let Some(set) = block_on(async { LUMA.write().await }).get_index_mut(tab) {
            set.1.insert(index, Link::new(name, link))
        }
    });

    LumaMessage::SetMode(Mode::Insert(vec![
        ("link: ".to_string(), String::new(), tx_link),
        ("name: ".into(), String::new(), tx_name),
    ]))
    .into()
}

fn move_down() -> Option<LumaMessage> {
    log::debug!("Moving cursor down");
    let mut state = block_on(async { STATE.write().await });

    let max_len = block_on(async { LUMA.read().await })
        .get_index(state.selected_tab)
        .unwrap()
        .1
        .len();
    if state.selected_index < max_len - 1 {
        state.selected_index += 1;
    }

    LumaMessage::Redraw.into()
}

fn move_up() -> Option<LumaMessage> {
    let mut state = block_on(async { STATE.write().await });

    if state.selected_index > 0 {
        state.selected_index -= 1;
    }
    LumaMessage::Redraw.into()
}

fn select() -> Option<LumaMessage> {
    let (tab, index) = block_on(util::get_tab_and_index());
    {
        let luma = block_on(async { LUMA.read().await });
        let set = luma.get_index(tab).unwrap();
        let link = set.1.get(index).unwrap();
        log::info!("Opening link: {}", link.link);
        AUDIO_OPENER.open(&link.link);
    }
    None
}

fn go_top() -> Option<LumaMessage> {
    let mut state = block_on(async { STATE.write().await });
    state.selected_index = 0;
    LumaMessage::Redraw.into()
}

fn go_bottom() -> Option<LumaMessage> {
    let mut state = block_on(async { STATE.write().await });
    let max_index = block_on(async { LUMA.read().await })
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

/// This currently works but is a bit hacky
/// There might be some consideration to using a tokio::sync::RwLock so the write
/// can be held over the await to garentee that no one changes it in the down time
fn delete() -> Option<LumaMessage> {
    let (tab, index) = block_on(util::get_tab_and_index());

    let name = block_on(async { LUMA.read().await })
        .get_index(tab)
        .unwrap()
        .1
        .get(index)
        .unwrap()
        .name
        .clone();

    let (tx, rx) = oneshot::channel();

    let _h = tokio::spawn(async move {
        if let Ok(PromptResponse::Yes) = rx.await {
            block_on(async { LUMA.write().await })
                .get_index_mut(tab)
                .unwrap()
                .1
                .remove(index);
        }
    });

    LumaMessage::SetMode(Mode::Prompt(crate::mode::PromptData {
        prompt: format!("Remove audio \"{}\"? (y/N)", name).into_boxed_str(),
        resp: Some(tx),
    }))
    .into()
}

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
