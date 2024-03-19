use crate::prelude::*;

#[derive(Debug)]
pub struct Terminal {
    stdout: std::io::Stdout,
    pub backend: tui::Terminal<tui::backend::CrosstermBackend<fs::File>>,
}

impl Terminal {
    /// Creates the terminal to render the tui to. You do not need to call init.
    /// It is done magically for you.
    pub fn new(stdout: fs::File) -> Self {
        let backend = tui::Terminal::new(tui::backend::CrosstermBackend::new(stdout)).unwrap();

        let stdout = std::io::stdout();

        let mut term = Self { stdout, backend };
        term.init();
        term
    }

    pub fn init(&mut self) {
        crossterm::terminal::enable_raw_mode().unwrap();
        crossterm::execute!(self.stdout, crossterm::terminal::EnterAlternateScreen).unwrap();
    }

    pub fn deinit(&mut self) {
        crossterm::execute!(self.stdout, crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.deinit()
    }
}
