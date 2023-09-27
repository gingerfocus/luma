use crate::prelude::*;
use std::rc::Rc;

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

use crate::{input::Key, mode::InsertData};

use super::Handler;

pub fn add_all(h: &mut Handler) {
    h.add_normal_handler(Key::Esc, nothing);

    h.add_normal_handlers([Key::Char('q'), Key::Ctrl('c')], exit);

    h.add_normal_handler(Key::Char('i'), go_insert);

    h.add_normal_handlers([Key::Down, Key::Char('j')], move_down);
    h.add_normal_handlers([Key::Up, Key::Char('k')], move_up);

    h.add_normal_handler(Key::Enter, select);

    h.add_normal_handler(Key::Char('g'), go_top);
    h.add_normal_handler(Key::Char('G'), go_bottom);

    h.add_normal_handlers([Key::Char('D'), Key::Backspace], delete);

    h.add_normal_handler(Key::Char('?'), show_help);
}

fn exit(_state: &mut Luma) -> Option<LumaMessage> {
    Some(LumaMessage::Exit)
}

fn nothing(_state: &mut Luma) -> Option<LumaMessage> {
    None
}

fn go_insert(_state: &mut Luma) -> Option<LumaMessage> {
    let read = STATE.read().unwrap();
    let tab = read.selected_tab;
    let index = read.selected_tab;
    drop(read);

    let callback = Rc::new(move |luma: &mut Luma, buffers: Vec<String>| {
        let link = &buffers[0];
        let name = &buffers[1];

        luma.get_index_mut(tab)
            .unwrap()
            .1
            .insert(index, Link::new(name, link));
    });

    let prompts = ["link".into(), "name".into(), "".into(), "".into()];
    Some(LumaMessage::SetMode(Mode::Insert(
        InsertData::new(prompts, callback).unwrap(),
    )))
}

fn move_down(state: &mut Luma) -> Option<LumaMessage> {
    let mut gwrite = STATE.write().unwrap();

    let max_len = state.get_index(gwrite.selected_tab).unwrap().1.len();
    if gwrite.selected_index < max_len {
        gwrite.selected_index += 1;
    }
    Some(LumaMessage::Redraw)
}

fn move_up(_state: &mut Luma) -> Option<LumaMessage> {
    let mut gwrite = STATE.write().unwrap();

    if gwrite.selected_index > 0 {
        gwrite.selected_index -= 1;
    }
    Some(LumaMessage::Redraw)
}

fn select(luma: &mut Luma) -> Option<LumaMessage> {
    let gread = STATE.read().unwrap();
    let tab = gread.selected_tab;
    let index = gread.selected_index;
    drop(gread);

    let set = luma.get_index(tab)?;
    let link = set.1.get(index)?;
    crate::AUDIO_OPENER.open(&link.link);

    // No need to redraw
    None
}

fn go_top(_luma: &mut Luma) -> Option<LumaMessage> {
    STATE.write().unwrap().selected_index = 0;
    Some(LumaMessage::Redraw)
}

fn go_bottom(luma: &mut Luma) -> Option<LumaMessage> {
    let mut gwrite = STATE.write().unwrap();
    let max_index = luma.get_index(gwrite.selected_tab)?.1.len();
    gwrite.selected_index = max_index - 1;
    Some(LumaMessage::Redraw)
}

const HELP_TEXT: &'static str = "\
j k     -       Move Up/Down\n\
g G     -       Go Top/Bottom\n\
Enter   -       Open Link in Browser\n\
c-j c-k -       Move Link Up/Down\n\
i       -       Add Link\n\
D       -       Delete entry\n\
q       -       Exit\
";

fn show_help(_luma: &mut Luma) -> Option<LumaMessage> {
    let nothing = |_luma: &mut Luma| {};
    Some(LumaMessage::SetMode(Mode::Prompt(
        crate::mode::PromptData {
            prompt: HELP_TEXT.to_string().into_boxed_str(),
            accepted: Rc::new(nothing),
            declined: Rc::new(nothing),
        },
    )))
}

fn delete(luma: &mut Luma) -> Option<LumaMessage> {
    let gread = STATE.read().unwrap();
    let tab = gread.selected_tab;
    let index = gread.selected_index;
    drop(gread);

    let set = luma.get_index(tab)?.1;
    let link = set.get(index)?;
    let name = link.name.clone();

    let delete = move |luma: &mut Luma| {
        luma.get_index_mut(tab).expect("thing").1.remove(index);
    };
    let nothing = |_luma: &mut Luma| {};

    Some(LumaMessage::SetMode(Mode::Prompt(
        crate::mode::PromptData {
            prompt: format!("Remove audio \"{}\"? (y/N)", name).into_boxed_str(),
            accepted: Rc::new(delete),
            declined: Rc::new(nothing),
        },
    )))
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
