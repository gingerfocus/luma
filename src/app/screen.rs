use tuirealm::tui::prelude::*;

use crate::prelude::*;

pub struct Screen {
    configured_size: (u16, u16),

    pub tab_bar: Rect,
    pub tree_veiw: Rect,
    pub preview: Rect,
}

// ListState

impl Screen {
    pub fn new(x: u16, y: u16) -> Self {
        let mut screen = Self {
            configured_size: (0, 0),
            tab_bar: default(),
            tree_veiw: default(),
            preview: default()
        };
        screen.configure_surface(x, y);
        screen
    }

    pub fn configure_surface(&mut self, x: u16, y: u16) {
        if self.configured_size != (x, y) {
            self.configured_size = (x, y);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
                .split(Rect::new(0, 0, x, y));

            self.tab_bar = chunks[0];

            let div = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Min(1)].as_ref())
                .split(chunks[1]);

            self.tree_veiw = div[0];
            self.preview = div[1];
        }
    }
}
