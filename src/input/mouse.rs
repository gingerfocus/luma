use crossterm::event;


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

impl From<event::MouseEvent> for Mouse {
    fn from(value: event::MouseEvent) -> Self {
        let event::MouseEvent {
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
