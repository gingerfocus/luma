use crate::prelude::*;

#[derive(Debug)]
pub struct Terminal {
    stdout: std::io::Stdout,
    pub backend: tui::Terminal<tui::backend::CrosstermBackend<fs::File>>,
}

impl Terminal {
    pub fn new(stdout: fs::File) -> Self {
        let backend = tui::Terminal::new(tui::backend::CrosstermBackend::new(stdout)).unwrap();

        let mut stdout = std::io::stdout();
        crossterm::terminal::enable_raw_mode().unwrap();
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen).unwrap();

        Self { stdout, backend }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::execute!(self.stdout, crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}
