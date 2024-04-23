use crate::{
    app::{Item, Mode},
    event::Key,
    input::Msg,
};

pub fn handle(key: Key, item: &Item) -> Option<Msg> {
    let msg = match key {
        Key::Char('y') | Key::Char('Y') => match item {
            Item::Tab => Msg::DeleteTab,
            Item::Link => Msg::Delete,
        },
        Key::Char('n') | Key::Char('N') | Key::Ctrl('c') | Key::Esc | Key::Enter => {
            Msg::ChangeMode(Mode::Normal)
        }
        _ => return None,
    };
    Some(msg)
}
