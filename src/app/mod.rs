mod delete;
mod normal;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::event::Event; // use crossterm::event::Event;
use crate::input::Msg;
use crate::prelude::*;

use crate::state::Link;

#[derive(Debug)]
pub struct App<B: tui::backend::Backend + io::Write> {
    /// Handle to the terminal
    pub term: tui::Terminal<B>,
    /// The main state of the app.
    luma: Luma,
    /// State of the applications
    pub state: State,
    /// A list or process that are loosly tied to the app
    task: Vec<std::process::Child>,
}

#[derive(Debug, Default)]
pub struct State {
    /// If the app should exit.
    pub quit: bool,
    /// If the view should render blank before drawing again
    pub redraw: bool,
    /// Selected tab
    pub tabb: usize,
    /// Selected index
    pub selected: usize,
    /// Ofset of the the display
    pub ofst: usize,
    /// Mode of display that the terminal is in
    pub mode: Mode,
}

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Normal,
    // Prompt(PromptData),
    /// The app should prompt the user if they want to delete a thing
    Delete(Item),
}

#[derive(Debug)]
pub enum Item {
    Tab,
    Link,
}

#[derive(Debug)]
pub enum AppError {
    Draw,
    Edit,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Draw => f.write_str("failed to draw the screen"),
            AppError::Edit => f.write_str("failed to edit the link"),
        }
    }
}
impl Context for AppError {}

impl App<tui::backend::CrosstermBackend<fs::File>> {
    pub fn new(luma: Luma, stdout: fs::File) -> Self {
        Self {
            // Creates the terminal to render the tui to.
            term: tui::Terminal::new(tui::backend::CrosstermBackend::new(stdout)).unwrap(),
            luma,
            state: State::default(),
            task: Vec::new(),
        }
    }
}

impl<B: tui::backend::Backend + io::Write> App<B> {
    pub fn draw(&mut self) -> Result<(), AppError> {
        if self.state.redraw {
            self.term
                .draw(|f| f.render_widget(tui::widgets::Clear, f.size()))
                .change_context(AppError::Draw)?;

            self.state.redraw = false;
        }

        self.term
            .draw(|f| match self.state.mode {
                Mode::Normal => normal::draw(f, &self.luma, &self.state),
                Mode::Delete(_) => delete::draw(f, &self.luma, &self.state),
            })
            .change_context(AppError::Draw)?;

        log::debug!("frame finished");

        Ok(())
    }

    pub fn event(&mut self, event: impl Into<Event>) -> Result<(), AppError> {
        // testing only
        match event.into() {
            Event::Input(k) => {
                let msg = match &self.state.mode {
                    Mode::Normal => crate::input::normal::handle(k, &mut self.state),
                    Mode::Delete(item) => crate::input::delete::handle(k, item),
                };
                if let Some(msg) = msg {
                    log::info!("msg: {:?}", msg);
                    self.handle(msg)?;
                }
            }
            // Event::Click(_) => {}
            // Event::Resize(_, _) => self.state.redraw = true,
            _ => {}
        }
        Ok(())
    }

    fn handle(&mut self, msg: Msg) -> Result<(), AppError> {
        match msg {
            Msg::Quit => self.state.quit = true,
            Msg::Edit => {
                if let Some(link) = self.luma.get_mut_selected(&self.state) {
                    edit(self.term.backend_mut(), link)?;
                    self.state.redraw = true;
                }
            }
            Msg::MoveDown(s) => {
                self.state.selected = self.state.selected.saturating_add(s);
                self.state.selected = self.state.selected.min(
                    self.luma
                        .tabs
                        .get(self.state.tabb)
                        .unwrap()
                        .1
                        .len()
                        .saturating_sub(1),
                );
            }
            Msg::MoveUp(s) => {
                self.state.selected = self.state.selected.saturating_sub(s);
            }
            Msg::ChangeMode(m) => {
                self.state.mode = m;
            }
            Msg::Open => {
                if let Some(s) = self.luma.get_selected(&self.state) {
                    match crate::prelude::LINK_OPENER.run(s.link.as_str()) {
                        Ok(c) => self.task.push(c),
                        Err(e) => log::warn!("failed to open link: {:?}", e),
                    }
                }
            }
            Msg::Delete => {
                self.delete();
                self.state.mode = Mode::Normal;
            }

            // -------------- changing tabs ------------------
            Msg::SelectTab(t) => {
                if t == self.state.tabb {
                    return Ok(());
                }
                if let Some(tab) = self.luma.tabs.get(t) {
                    self.state.tabb = t;

                    if self.state.selected >= tab.1.len() {
                        self.state.selected = tab.1.len().saturating_sub(1);
                    }
                }
            }
            // -----------------------------------------------
            //
            Msg::Add => {
                if let Some(tabb) = self.luma.tabs.get_mut(self.state.tabb) {
                    let mut link = Link::default();
                    edit(self.term.backend_mut(), &mut link)?;
                    self.state.redraw = true;
                    tabb.1.push(link);
                }
            }
            Msg::RenameTab => {
                if let Some(tabb) = self.luma.tabs.get_mut(self.state.tabb) {
                    let name = &mut tabb.0;
                    edit(self.term.backend_mut(), name)?;
                    self.state.redraw = true;
                }
            }
            Msg::DeleteTab => {
                self.luma.tabs.remove(self.state.tabb);
                self.state.tabb = self.state.tabb.saturating_sub(1);
                self.state.redraw = true;
                self.state.mode = Mode::Normal;
            }
            Msg::AddTab => {
                let mut name = String::new();
                edit(self.term.backend_mut(), &mut name)?;
                self.luma.tabs.push((name, Vec::new()));
                self.state.tabb = self.luma.tabs.len() - 1;
                self.state.redraw = true;
            }
        }
        Ok(())
    }

    fn delete(&mut self) {
        if let Some(tabb) = self.luma.tabs.get_mut(self.state.tabb) {
            if self.state.selected >= tabb.1.len() {
                return;
            }
            tabb.1.remove(self.state.selected);
            self.state.selected = self.state.selected.saturating_sub(1);
        }
    }

    pub fn finish(self) -> (Luma, tui::Terminal<B>) {
        (self.luma, self.term)
    }
}

/// Edits an item
fn edit<B: std::io::Write, T: Serialize + DeserializeOwned>(
    term: &mut B,
    item: &mut T,
) -> Result<(), AppError> {
    deinit(term);

    let mut f = tempfile::NamedTempFile::new()
        .change_context(AppError::Edit)
        .attach_printable("could not create temp file")?;

    let p = f.path();
    let file = p.to_str().unwrap().to_owned();

    log::debug!("tempfile path: {:?}", file);

    json::to_writer_pretty(&mut f, item).unwrap();

    let mut ch = crate::prelude::TEXT_OPENER
        .spawn(&file)
        .change_context(AppError::Edit)
        .attach_printable("could spawn editor")?;

    let e = ch
        .wait()
        .change_context(AppError::Edit)
        .attach_printable("could not wait for editor")?;

    if e.success() {
        io::Seek::seek(&mut f, io::SeekFrom::Start(0))
            .change_context(AppError::Edit)
            .attach_printable("could not seek to start of file")?;

        let nval = json::from_reader(f)
            .change_context(AppError::Edit)
            .attach_printable("input was no of valid form")?;

        *item = nval;
    }

    init(term);

    Ok(())
}

pub fn init<B: io::Write>(write: &mut B) {
    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::execute!(write, crossterm::terminal::EnterAlternateScreen).unwrap();
}

pub fn deinit<B: io::Write>(b: &mut B) {
    crossterm::execute!(
        b,
        crossterm::terminal::LeaveAlternateScreen,
        // crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
    )
    .unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
}

// #[cfg(test)]
// mod test {
//     use std::io;
//
//     pub struct TestBackend {
//         inner: tui::backend::TestBackend,
//     }
//
//     impl TestBackend {
//         pub fn new(width: u16, height: u16) -> Self {
//             Self {
//                 inner: tui::backend::TestBackend::new(width, height),
//             }
//         }
//     }
//
//     impl tui::backend::Backend for TestBackend {
//         fn draw<'a, I>(&mut self, content: I) -> std::io::Result<()>
//         where
//             I: Iterator<Item = (u16, u16, &'a tui::buffer::Cell)>,
//         {
//             self.inner.draw(content)
//         }
//
//         fn hide_cursor(&mut self) -> std::io::Result<()> {
//             self.inner.hide_cursor()
//         }
//
//         fn show_cursor(&mut self) -> std::io::Result<()> {
//             self.inner.show_cursor()
//         }
//
//         fn get_cursor(&mut self) -> std::io::Result<(u16, u16)> {
//             self.inner.get_cursor()
//         }
//
//         fn set_cursor(&mut self, x: u16, y: u16) -> std::io::Result<()> {
//             self.inner.set_cursor(x, y)
//         }
//
//         fn clear(&mut self) -> std::io::Result<()> {
//             self.inner.clear()
//         }
//
//         fn size(&self) -> std::io::Result<tui::layout::Rect> {
//             self.inner.size()
//         }
//
//         fn window_size(&mut self) -> std::io::Result<tui::backend::WindowSize> {
//             self.inner.window_size()
//         }
//
//         fn flush(&mut self) -> std::io::Result<()> {
//             self.inner.flush()
//         }
//     }
//     impl io::Write for TestBackend {
//         fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//             Ok(buf.len())
//         }
//
//         fn flush(&mut self) -> io::Result<()> {
//             Ok(())
//         }
//     }
//
//     #[test]
//     fn quit() {
//         let mut app = super::App {
//             term: tui::Terminal::new(TestBackend::new(10, 10)).unwrap(),
//             luma: crate::Luma::default(),
//             state: super::State::default(),
//             task: Vec::new(),
//         };
//         app.handle(super::Msg::Quit).unwrap();
//
//         assert!(app.state.quit);
//     }
// }
