use crossterm::event;
use crossterm::event::Event as CrossEvent;
use crossterm::event::MouseEvent;
use std::fmt;

/// An occurred event.
#[derive(Default, Debug)]
pub enum Event {
    /// An input event occurred.
    Input(Key),
    /// When the mouse is clicked
    Click(Mouse),
    /// when the terminal is resized
    Resize(u16, u16),
    /// when some text is put into the terminal
    Paste(String),
    /// A change in focus for the screen, true when gained, false when lost
    GainedFocus(bool),
    /// An tick event occurred.
    #[default]
    Tick,
}

impl From<CrossEvent> for Event {
    fn from(value: crossterm::event::Event) -> Self {
        match value {
            CrossEvent::FocusGained => Event::GainedFocus(true),
            CrossEvent::FocusLost => Event::GainedFocus(false),
            CrossEvent::Key(ke) => Event::Input(ke.into()),
            CrossEvent::Mouse(me) => Event::Click(me.into()),
            CrossEvent::Paste(s) => Event::Paste(s),
            CrossEvent::Resize(x, y) => Event::Resize(x, y),
        }
    }
}

/// Represents an key.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    /// Both Enter (or Return) and numpad Enter
    Enter,
    /// Tabulation key
    Tab,
    /// Backspace key
    Backspace,
    /// Escape key
    Esc,

    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,

    /// Insert key
    Ins,
    /// Delete key
    Delete,
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,

    /// F key
    F(u8),
    Char(char),
    Ctrl(char),
    Alt(char),
    Unknown,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Key::Alt(' ') => write!(f, "<Alt+Space>"),
            Key::Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            Key::Char(' ') => write!(f, "<Space>"),
            Key::Alt(c) => write!(f, "<Alt+{}>", c),
            Key::Ctrl(c) => write!(f, "<Ctrl+{}>", c),
            Key::Char(c) => write!(f, "{}", c),
            Key::Left | Key::Right | Key::Up | Key::Down => write!(f, "<{:?} Arrow Key>", self),
            Key::Enter
            | Key::Tab
            | Key::Backspace
            | Key::Esc
            | Key::Ins
            | Key::Delete
            | Key::Home
            | Key::End
            | Key::PageUp
            | Key::PageDown => write!(f, "<{:?}>", self),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } => Key::Esc,
            event::KeyEvent {
                code: event::KeyCode::Backspace,
                ..
            } => Key::Backspace,
            event::KeyEvent {
                code: event::KeyCode::Left,
                ..
            } => Key::Left,
            event::KeyEvent {
                code: event::KeyCode::Right,
                ..
            } => Key::Right,
            event::KeyEvent {
                code: event::KeyCode::Up,
                ..
            } => Key::Up,
            event::KeyEvent {
                code: event::KeyCode::Down,
                ..
            } => Key::Down,
            event::KeyEvent {
                code: event::KeyCode::Home,
                ..
            } => Key::Home,
            event::KeyEvent {
                code: event::KeyCode::End,
                ..
            } => Key::End,
            event::KeyEvent {
                code: event::KeyCode::PageUp,
                ..
            } => Key::PageUp,
            event::KeyEvent {
                code: event::KeyCode::PageDown,
                ..
            } => Key::PageDown,
            event::KeyEvent {
                code: event::KeyCode::Delete,
                ..
            } => Key::Delete,
            event::KeyEvent {
                code: event::KeyCode::Insert,
                ..
            } => Key::Ins,
            event::KeyEvent {
                code: event::KeyCode::F(n),
                ..
            } => Key::F(n),
            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,
            event::KeyEvent {
                code: event::KeyCode::Tab,
                ..
            } => Key::Tab,

            // First check for char + modifier
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::ALT,
                ..
            } => Key::Alt(c),
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Key::Ctrl(c),
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => Key::Char(c),
            _ => Key::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Mouse {
    pub pos: (u16, u16),
    pub kind: MouseKind,
}

#[derive(Debug)]
pub enum MouseKind {
    LeftClick,
    RightClick,
    MiddleClick,
    Drag,
    Scroll(i8),
    Nothing,
}

impl From<MouseEvent> for Mouse {
    fn from(value: MouseEvent) -> Self {
        let MouseEvent {
            kind, column, row, ..
        } = value;

        let kind = match kind {
            event::MouseEventKind::Down(m) => match m {
                event::MouseButton::Left => MouseKind::LeftClick,
                event::MouseButton::Right => MouseKind::RightClick,
                event::MouseButton::Middle => MouseKind::MiddleClick,
            },
            event::MouseEventKind::ScrollDown => MouseKind::Scroll(1),
            event::MouseEventKind::ScrollUp => MouseKind::Scroll(-1),
            event::MouseEventKind::Drag(event::MouseButton::Left) => MouseKind::Drag,

            // event::MouseEventKind::Up(_) => MouseKind::Nothing,
            // event::MouseEventKind::Drag(_) => MouseKind::Nothing,
            // event::MouseEventKind::Moved => MouseKind::Nothing,
            // event::MouseEventKind::ScrollLeft => MouseKind::Nothing,
            // event::MouseEventKind::ScrollRight => MouseKind::Nothing,
            _ => MouseKind::Nothing,
        };

        Self {
            pos: (row, column),
            kind,
        }
    }
}
