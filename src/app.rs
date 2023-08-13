use crate::{event::Key, prelude::*};

#[derive(Debug)]
pub struct App {
    pub run: bool,
    is_valid: bool,
    stdout: io::Stdout,
    root: Node,
    mode: Mode,
}

#[derive(Debug, Default)]
enum Mode {
    #[default]
    Normal,
    Insert,
    Visual,
}

impl App {
    pub fn new(root: Node) -> Self {
        Self {
            run: true,
            is_valid: false,
            stdout: io::stdout(),
            root,
            mode: Mode::Normal,
        }
    }

    pub fn read_event(&self) -> Event {
        if crossterm::event::poll(std::time::Duration::from_millis(500)).unwrap() {
            crossterm::event::read().map(Into::into).unwrap()
        } else {
            Event::Tick
        }
    }

    pub fn render(&mut self) {
        crossterm::execute!(
            self.stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();
        _ = crossterm::terminal::disable_raw_mode();
        println!("{}", self.root.to_string());
        _ = crossterm::terminal::enable_raw_mode();

        //     markdown
        //         .children()
        //         .unwrap()
        //         .iter()
        //         .flat_map(|child| {
        //             if let Node::Link(l) = child {
        //                 Some(l)
        //             } else {
        //                 None
        //             }
        //         })
        //         .skip(starting)
        //         .take(height - 1)
        //         .enumerate()
        //         .for_each(|(i, child)| {
        //             execute!(stdout, cursor::MoveTo(0, print as u16)).unwrap();
        //             let t = &child.title;
        //             let text = t.clone().unwrap();
        //             print!("{}: {}", i + starting, text);
        //         });
        //
        //     let cursor_pos = std::cmp::min(index as u16, 5);
        //     execute!(stdout, cursor::MoveTo(0, cursor_pos)).unwrap();
    }

    pub fn handle(&mut self, event: Event) {
        match event {
            Event::GainedFocus(_did) => todo!(),
            Event::Input(key) => self.handle_key(key),
            Event::Click(_me) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
            Event::Tick => {}
        }
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
            Key::Down | Key::Char('j') => todo!(),
            Key::Up | Key::Char('k') => todo!(),
            Key::Right | Key::Char('l') => todo!(),
            Key::Char('i') => self.mode = Mode::Insert,

            Key::Char('g') => todo!("go to the top"),
            Key::Char('G') => todo!("go to the bottom"),

            Key::Char('D') => todo!("Prompt to delete line"),

            Key::Char('z') => todo!("fold selected section"),
            Key::Enter => todo!("open selected link or move 1 line down"),

            Key::Ctrl('j') => todo!("move line up 1"),
            Key::Ctrl('k') => todo!("move line down 1"),
            Key::Tab => todo!(),
            Key::Backspace => todo!(),
            Key::Alt('e') => todo!("enter search"),
            _ => todo!(),
        }
    }
    pub fn can_run(&self) -> bool {
        self.run && self.is_valid
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        crossterm::execute!(self.stdout, crossterm::terminal::EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;
        self.is_valid = true;
        Ok(())
    }

    pub fn deinit(&mut self) -> anyhow::Result<()> {
        if self.is_valid {
            crossterm::terminal::disable_raw_mode()?;
            crossterm::execute!(self.stdout, crossterm::terminal::LeaveAlternateScreen)?;
            // crossterm::execute!(stdout, Clear(ClearType::All))?;
            self.is_valid = false;
        }
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.deinit().unwrap();
    }
}
