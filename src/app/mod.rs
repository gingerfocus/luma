mod normal;

use crate::input::Msg;
use crate::prelude::*;

use crate::ui::Terminal;

#[derive(Debug)]
pub struct App {
    /// Handle to the terminal
    term: Terminal,
    /// The main state of the app.
    luma: Luma,
    /// State of the applications
    pub stat: State,
}

#[derive(Debug, Default)]
pub struct State {
    /// If the app should exit.
    pub quit: bool,
    /// If the app should draw.
    pub draw: bool,
    /// Selected tab
    pub tabb: usize,
    /// Selected index
    pub selc: usize,
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
    pub fn new(luma: Luma, stdout: fs::File) -> Self {
        Self {
            term: Terminal::new(stdout),
            luma,
            stat: State::default(),
        }
    }

    pub fn draw(&mut self) -> Result<(), AppError> {
        self.term
            .backend
            .draw(|f| match self.stat.mode {
                Mode::Normal => normal::draw(f, &self.luma, &self.stat),
            })
            .change_context(AppError::Draw)?;

        log::debug!("frame finished");

        Ok(())
    }

    pub fn redraw(&mut self) -> Result<(), AppError> {
        self.term
            .backend
            .draw(|f| f.render_widget(tui::widgets::Clear, f.size()))
            .change_context(AppError::Draw)?;

        self.draw()
    }

    pub fn event(&mut self, event: crossterm::event::Event) {
        // testing only
        match event {
            crossterm::event::Event::Key(k) => {
                let msg = match self.stat.mode {
                    Mode::Normal => crate::input::normal::handle(k.into(), &mut self.stat),
                };
                self.handle(msg);
            }

            crossterm::event::Event::Paste(_) => {}
            crossterm::event::Event::Resize(_, _) => self.stat.draw = true,
            // I do not care
            crossterm::event::Event::Mouse(_) => {}
            crossterm::event::Event::FocusGained => {}
            crossterm::event::Event::FocusLost => {}
        }
    }

    pub fn handle(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => todo!(),
            Msg::Edit => self.edit(),
        }
    }

    pub fn edit(&mut self) {
        self.term.deinit();
        self.edit_raw();
        self.term.init();
        self.redraw().unwrap();
    }

    /// Edits the currently selected link
    fn edit_raw(&mut self) {
        let a = self.luma.tabs.get_mut(self.stat.tabb).unwrap();
        let b = &mut a.1;
        let c = b.get_mut(self.stat.selc).unwrap();

        let mut f = tempfile::NamedTempFile::new().unwrap();
        let p = f.path();
        log::info!("path: {:?}", p);
        let file = p.to_str().unwrap().to_owned();
        yaml::to_writer(&mut f, c).unwrap();
        let mut ch = crate::prelude::TEXT_OPENER.spawn(&file).unwrap();

        let e = ch.wait().unwrap();

        if e.success() {
            let s = fs::File::open(&file).unwrap();
            let l = yaml::from_reader(s).unwrap();
            *c = l;
        }
    }

    pub fn finish(self) -> (Luma, Terminal) {
        (self.luma, self.term)
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
