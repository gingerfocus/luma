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

    state: ScreenState,
    // cache: Components
}

#[derive(Default)]
struct ScreenState {
    pub selected_tab: IndexedPair,

    active: ScreenStatefulData,
}

#[derive(Default)]
pub struct IndexedPair {
    pub name: String,
    pub index: usize,
}

#[derive(Default)]
struct ScreenStatefulData {
    list: ListState,
}
// struct Components {
//     list: List
// }

impl Screen {
    pub fn init(&mut self) -> anyhow::Result<()> {
        if !self.is_valid {
            crossterm::terminal::enable_raw_mode().unwrap();
            crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

            self.state.active.list.select(Some(0));

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

    pub fn move_cursor(&mut self, step: i8) {
        let select = match step {
            0 => return,
            x if x > 0 => match self.state.active.list.selected() {
                None => 0,
                // x is gaurded against being negative so casts always sucessed
                Some(i) => i + x as usize,
            },
            x => match self.state.active.list.selected() {
                None => 0,
                // x is negative is the inverse will always cast sucessfully
                Some(i) => i.saturating_sub(-x as usize),
            },
        };

        self.state.active.list.select(Some(select));
    }

    pub fn select_tab(&mut self, index: IndexedPair) {
        self.state.selected_tab = index;
        // for now the best that can be done is to select none but Some(0)
        // could be selected if the luma was passed in
        self.state.active.list.select(None);
    }

    pub fn get_selected_tab_index(&self) -> usize {
        self.state.selected_tab.index
    }

    pub fn select_index(&mut self, index: usize) {
        // TODO: Bounds checks
        self.state.active.list.select(Some(index));
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.state.active.list.selected()
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.state.active.list
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
        screen.move_cursor(4);

        assert!(screen.get_selected_index() == Some(12));

        screen.select_index(1);
        screen.move_cursor(-3);

        assert!(screen.get_selected_index() == Some(0));
    }
}
