mod normal;

use crate::prelude::*;

#[derive(Debug)]
pub struct App {
    /// If the app should exit.
    pub quit: bool,
    /// If the app should draw.
    pub draw: bool,
    /// Handle to the terminal
    term: Terminal,
    /// The main state of the app.
    luma: Luma,
    /// Mode of display that the terminal is in
    mode: Mode,
}

#[derive(Default, Debug)]
pub enum Mode {
    #[default]
    Normal,
    // Prompt(PromptData),
}

#[derive(Debug)]
pub enum AppError {
    Draw,
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Draw => f.write_str("failed to draw the screen"),
        }
    }
}
impl Context for AppError {}

impl App {
    pub fn new(luma: Luma) -> Self {
        Self {
            quit: false,
            draw: true,
            term: Terminal::new(),
            mode: Mode::default(),
            luma,
        }
    }

    pub fn redraw(&mut self) -> Result<(), AppError> {
        log::debug!("thing 2.");

        self.term
            .backend
            .draw(|f| match self.mode {
                Mode::Normal => normal::draw(f, &self.luma),
            })
            .change_context(AppError::Draw)?;

        log::debug!("frame finished");

        Ok(())
    }

    pub fn event(&mut self, event: crossterm::event::Event) {
        // testing only
        match event {
            crossterm::event::Event::Key(_) => self.quit = true,
            crossterm::event::Event::Paste(_) => {}

            crossterm::event::Event::Resize(_, _) => self.draw = true,

            // I do not care
            crossterm::event::Event::Mouse(_) => {}
            crossterm::event::Event::FocusGained => {}
            crossterm::event::Event::FocusLost => {}
        }
    }
}

#[derive(Debug)]
pub struct Terminal {
    stdout: std::io::Stdout,
    pub backend: tui::Terminal<tui::backend::CrosstermBackend<io::Stdout>>,
}

impl Terminal {
    fn new() -> Self {
        let backend =
            tui::Terminal::new(tui::backend::CrosstermBackend::new(std::io::stdout())).unwrap();

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

//     pub fn draw(&mut self, mode: &Mode) -> Result<()> {
//         log::debug!("redraw started");
//         self.terminal.draw(|f| {
//             // configure the surface so out drawing boxes are the right size
//             // if they changed since last render.
//             let Rect { width, height, .. } = f.size();
//             self.screen.configure_surface(width, height);
//
//             let (tab, index) = futures::executor::block_on(util::get_tab_and_index());
//             log::trace!("selected tab is: {}", tab);
//             log::trace!("selected index is: {}", index);
//
//             {
//                 // let luma = block_on(async { LUMA.read().await });
//                 //
//                 // let set = luma.get_index(tab).expect("A valid tab is not selected");
//                 //
//                 // if let Some(link) = set.1.get(index) {
//                 //     // TODO: when deleting the last item the cursor hovers nothing
//                 //
//                 //     let preview = link.as_paragraph();
//                 //     f.render_widget(preview, self.screen.preview_pane);
//                 // }
//                 //
//                 // let list = crate::ui::render::list(set.1);
//                 //
//                 // let mut state = ListState::default();
//                 // state.select(Some(index));
//                 // f.render_stateful_widget(list, self.screen.side_pane, &mut state);
//                 //
//                 // let names = luma.keys();
//                 // let tabs = crate::ui::render::tabs(names, tab);
//                 // f.render_widget(tabs, self.screen.title_bar);
//             }
//
//             match mode {
//                 Mode::Normal => {}
//                 Mode::Prompt(_data) => {
//                     // let prompt_render = crate::ui::render::prompt(&data.prompt);
//                     // let float_box = crate::ui::render::float_box(&data.prompt, width, height);
//
//                     // f.render_widget(Clear, float_box);
//                     // f.render_widget(prompt_render, float_box);
//                 }
//             }
//         })?;
