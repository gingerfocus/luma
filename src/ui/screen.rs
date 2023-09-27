use std::io;

use crate::prelude::*;
use tui::prelude::*;

#[derive(Default)]
pub struct Screen {
    pub is_valid: bool,
    configured_size: (u16, u16),

    pub title_bar: Rect,
    pub side_pane: Rect,
    pub preview_pane: Rect,
}

// ListState

impl Screen {
    pub fn init(&mut self) -> Result<()> {
        if !self.is_valid {
            crossterm::terminal::enable_raw_mode().unwrap();
            crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

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

    pub fn deinit(&mut self) -> Result<()> {
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
