use std::io;

// use crate::prelude::*;
use tui::prelude::*;
use tui::widgets::*;

#[derive(Default)]
pub struct Screen {
    pub is_valid: bool,
    configured_size: (u16, u16),

    pub title_bar: Rect,
    pub side_pane: Rect,
    pub preview_pane: Rect,

    active: ScreenState,
}

#[derive(Default)]
struct ScreenState {
    list: ListState,
    tab: ScreenType,
}

#[derive(Default, Clone, Copy)]
pub enum ScreenType {
    #[default]
    Audio,
    Reading, // StarterLinks,
}

impl From<ScreenType> for usize {
    fn from(value: ScreenType) -> Self {
        match value {
            ScreenType::Audio => 0,
            ScreenType::Reading => 1,
        }
    }
}

impl Screen {
    pub fn init(&mut self) -> anyhow::Result<()> {
        if !self.is_valid {
            crossterm::terminal::enable_raw_mode().unwrap();
            crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

            self.active.list.select(Some(0));

            self.is_valid = true;
        }
        Ok(())
    }

    pub fn configure_surface(&mut self, x: u16, y: u16) {
        if self.configured_size != (x, y) {
            self.configured_size = (x, y);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
                .split(Rect::new(0, 0, x, y));

            self.title_bar = chunks[0];

            let div = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Min(1)].as_ref())
                .split(chunks[1]);

            self.side_pane = div[0];
            self.preview_pane = div[1];
        }
    }

    /// Moves the cursor up 1 space respecting the given maximum for its index
    pub fn move_down(&mut self, max: usize) {
        let select = match self.active.list.selected() {
            None => 0,
            Some(i) if i >= max => i,
            Some(i) => i + 1,
        };
        self.active.list.select(Some(select));
    }

    /// Moves the cursor up 1 space.
    pub fn move_up(&mut self) {
        let select = match self.active.list.selected() {
            None => 0,
            Some(i) if i == 0 => 0,
            Some(i) => i - 1,
        };
        self.active.list.select(Some(select));
    }

    pub fn select_tab(&mut self, tab: ScreenType) {
        self.active.tab = tab;
        // for now the best that can be done is to select none but Some(0)
        // could be selected if the luma was passed in
        self.active.list.select(None);
    }

    pub fn get_selected_tab(&self) -> ScreenType {
        self.active.tab
    }

    pub fn select_index(&mut self, index: usize) {
        // TODO: Bounds checks
        self.active.list.select(Some(index));
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.active.list.selected()
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.active.list
    }

    pub fn deinit(&mut self) -> anyhow::Result<()> {
        if self.is_valid {
            // restore terminal, results are ignored beacuse we are leaving anyway
            crossterm::terminal::disable_raw_mode()?;
            crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
            // _ = self.terminal.show_cursor();
            self.is_valid = false;
        }
        Ok(())
    }
}

mod test {

    #[test]
    fn check_index_overflows() {
        let mut screen = super::Screen::default();

        screen.select_index(8);
        screen.move_down(8);

        assert!(screen.get_selected_index() == Some(8));

        screen.select_index(0);
        screen.move_up();

        assert!(screen.get_selected_index() == Some(0));
    }
}
