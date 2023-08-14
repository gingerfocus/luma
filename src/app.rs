use crate::{event::Key, prelude::*, screen::Screen};

#[derive(Default)]
pub struct App {
    pub run: bool,
    screen: Screen,
    mode: Mode,
    pub needs_redraw: bool,
}

#[derive(Debug, Default)]
enum Mode {
    #[default]
    Normal,
    Insert,
    Visual,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        self.screen.init()?;

        self.needs_redraw = true;
        self.run = true;

        Ok(())
    }

    // #[allow(unused)]
    // pub fn update_state(&mut self, _mpd_state: ClientState) {}
    //
    // fn handle_key_event(&mut self, key: Key, client: &mut Client) -> MumiRequest {
    //     match key {
    //         Key::Char('q') | Key::Ctrl('c') | Key::Esc => MumiRequest::Exit,
    //
    //         Key::Char('h') | Key::Left => self.screens.move_left(),
    //         Key::Char('j') | Key::Down => self.screens.move_down(),
    //         Key::Char('k') | Key::Up => self.screens.move_up(),
    //         Key::Char('l') | Key::Right => self.screens.move_up(),
    //
    //         Key::Char('p') => client.toggle(),
    //
    //         Key::Char('1') => self.screens.select_tab(0),
    //         Key::Char('2') => self.screens.select_tab(1),
    //         Key::Char('3') => self.screens.select_tab(2),
    //         Key::Char('4') => self.screens.select_tab(3),
    //         Key::Tab => self.screens.next_tab(),
    //         _k => MumiRequest::Nothing,
    //     }
    // }
    //
    // pub fn update(&mut self, event: Event, client: &mut Client) {
    //     let req = match event {
    //         Event::Input(ke) => self.handle_key_event(ke, client),
    //         Event::Resize(x, y) => self.screens.configure_surface(x, y),
    //         Event::GainedFocus(_did) => todo!(),
    //         Event::Click(_) => todo!(),
    //         Event::Paste(_) => todo!(),
    //         Event::Tick => MumiRequest::Nothing,
    //     };
    //     match req {
    //         MumiRequest::Nothing => {}
    //         MumiRequest::Redraw => self.needs_redraw = true,
    //         MumiRequest::Exit => self.running = false,
    //     }
    // }

    pub fn can_run(&self) -> bool {
        self.run && self.screen.is_valid
    }

    pub fn read_event(&self) -> Event {
        if crossterm::event::poll(std::time::Duration::from_millis(500)).unwrap() {
            crossterm::event::read().map(Into::into).unwrap()
        } else {
            Event::Tick
        }
    }

    pub fn redraw(&mut self, state: &State) {
        if self.needs_redraw {
            self.screen.draw(state);
            self.needs_redraw = false;
        }
    }

    pub fn handle(&mut self, event: Event, state: &mut State) {
        match event {
            Event::GainedFocus(_did) => todo!(),
            Event::Input(key) => self.handle_key(key),
            Event::Click(_me) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
            Event::Tick => {}
        }
        self.needs_redraw = true;
    }

    fn handle_key_insert(&mut self, key: Key) {
        match key {
            Key::Esc => self.mode = Mode::Normal,
            Key::Char(_c) => todo!(),
            _ => {}
        }
    }

    fn handle_key(&mut self, key: Key) {
        match self.mode {
            Mode::Normal => self.handle_key_normal(key),
            Mode::Insert => self.handle_key_insert(key),
            Mode::Visual => todo!("visual mode key handle"),
        }
    }

    fn handle_key_normal(&mut self, key: Key) {
        match key {
            Key::Char('q') | Key::Ctrl('c') => self.run = false,
            Key::Esc => {}
            Key::Left | Key::Char('h') => todo!(),
            Key::Down | Key::Char('j') => self.screen.move_down(),
            Key::Up | Key::Char('k') => self.screen.move_up(),
            Key::Right | Key::Char('l') => todo!(),
            Key::Char('i') => self.mode = Mode::Insert,

            Key::End => todo!("open selected link"),

            Key::Char('g') => todo!("go to the top"),
            Key::Char('G') => todo!("go to the bottom"),

            Key::Char('D') => todo!("Prompt to delete line"),

            Key::Char('z') => todo!("fold selected section"),
            Key::Enter => todo!("open selected link or move 1 line down"),

            Key::Ctrl('j') => todo!("move line up 1"),
            Key::Ctrl('k') => todo!("move line down 1"),

            Key::Char('1') => self.screen.selecte_tab(0),
            Key::Char('2') => self.screen.selecte_tab(1),

            Key::Tab => todo!(),
            Key::Backspace => todo!(),
            Key::Alt('e') => todo!("enter search"),
            _ => todo!(),
        }
    }

    pub fn deinit(&mut self) -> anyhow::Result<()> {
        self.screen.deinit()?;

        self.run = false;

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.deinit().unwrap();
    }
}
