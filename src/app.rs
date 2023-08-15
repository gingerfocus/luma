use std::{cell::RefCell, process::Stdio, rc::Rc};

use crate::{event::Key, prelude::*, state::Link, ui::Screen};

#[derive(Default)]
pub struct App {
    pub run: bool,
    screen: Screen,
    mode: Mode,
    // Will always be some when in prompt mode
    // prompt_callback: Option<Box<dyn Fn(&mut State) -> ()>>,

    // Will always be some when in prompt mode
    // insert_buffer: Option<String>,
}

#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    Prompt {
        msg: String,
        callback: Rc<dyn Fn(&mut State)>,
    },
    Insert {
        index: Rc<RefCell<usize>>,
        prompts: Rc<Vec<String>>,
        buffers: Rc<RefCell<Vec<String>>>,
        #[allow(clippy::type_complexity)]
        callback: Rc<dyn Fn(&mut State, Vec<String>)>,
    },
    // Visual,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        self.screen.init()?;

        self.run = true;

        Ok(())
    }

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

    pub fn draw(&mut self, state: &State) {
        if self.can_run() {
            self.screen.draw(state, &self.mode);
        }
    }

    pub fn deinit(&mut self) -> anyhow::Result<()> {
        self.screen.deinit()?;

        self.run = false;

        Ok(())
    }
}

impl App {
    pub fn handle(&mut self, event: Event, state: &mut State) {
        match event {
            Event::GainedFocus(_did) => todo!(),
            Event::Input(key) => self.handle_key(key, state),
            Event::Click(_me) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
            Event::Tick => {}
        }
    }

    fn handle_key(&mut self, key: Key, state: &mut State) {
        match &self.mode {
            Mode::Normal => self.handle_key_normal(key, state),
            Mode::Prompt { callback, .. } => self.handle_key_prompt(key, state, callback.clone()),
            Mode::Insert {
                index,
                buffers,
                callback,
                ..
            } => {
                self.handle_key_insert(key, state, index.clone(), buffers.clone(), callback.clone())
            }
        }
    }

    fn handle_key_prompt(&mut self, key: Key, state: &mut State, callback: Rc<dyn Fn(&mut State)>) {
        match key {
            Key::Char('y') | Key::Char('Y') => {
                // This should never panic as this should only ever be able to
                // be ran when it exists
                callback(state);
                self.mode = Mode::Normal;
            }

            Key::Enter | Key::Char('n') | Key::Char('N') | Key::Esc | Key::Ctrl('c') => {
                self.mode = Mode::Normal
            }
            _ => {}
        }
    }

    #[allow(clippy::type_complexity)]
    fn handle_key_insert(
        &mut self,
        key: Key,
        state: &mut State,
        index: Rc<RefCell<usize>>,
        buffers: Rc<RefCell<Vec<String>>>,
        callback: Rc<dyn Fn(&mut State, Vec<String>)>,
    ) {
        match key {
            Key::Esc | Key::Ctrl('c') => {
                self.mode = Mode::Normal;
            }
            Key::Char(ch) => buffers
                .borrow_mut()
                .get_mut(*index.borrow())
                .unwrap()
                .push(ch),
            Key::Enter => {
                let len = buffers.borrow().len();
                if *index.borrow() < len - 1 {
                    *index.borrow_mut() += 1;
                } else {
                    let buffers = buffers.take();
                    callback(state, buffers);
                    self.mode = Mode::Normal;
                }
            }
            Key::Backspace => _ = buffers.borrow_mut().get_mut(*index.borrow()).unwrap().pop(),
            _ => {}
        }
    }

    fn handle_key_normal(&mut self, key: Key, state: &mut State) {
        match key {
            Key::Char('q') | Key::Ctrl('c') => self.run = false,
            Key::Esc => {}
            Key::Left | Key::Char('h') => todo!(),
            Key::Down | Key::Char('j') => {
                let tab = self.screen.get_selected_tab();
                let len = state.set_mut(tab).len();
                self.screen.move_down(len);
            }
            Key::Up | Key::Char('k') => self.screen.move_up(),
            Key::Right | Key::Char('l') => todo!(),

            Key::Char('i') => {
                let index = self.screen.get_selected_index();
                let tab = self.screen.get_selected_tab();

                let callback = move |state: &mut State, mut buffers: Vec<String>| {
                    let name = buffers.pop().unwrap();
                    let link = buffers.pop().unwrap();

                    state.set_mut(tab).insert(index, Link::new(name, link));
                };

                self.mode = Mode::Insert {
                    index: Rc::new(RefCell::new(0)),
                    prompts: Rc::new(vec!["link".into(), "name".into()]),
                    buffers: Rc::new(RefCell::new(vec![String::new(), String::new()])),
                    callback: Rc::new(callback),
                }
            }

            // TODO: move one down if not in a state where something can be opened
            Key::Enter => {
                let set = self.screen.choose_set(state);
                let link = set.get(self.screen.get_selected_index()).expect("find it");
                std::process::Command::new(&state.meta.audio_open.cmd)
                    .args(&state.meta.audio_open.args)
                    // .args(vec![format!("--profile-directory={}", PROFILE_PATH)])
                    .arg(&link.link)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();
            }

            Key::Char('g') => self.screen.select_index(0),
            Key::Char('G') => {
                let tab = self.screen.get_selected_tab();
                let len = state.set_mut(tab).len();
                self.screen.select_index(len - 1);
            }

            // try creating some sort of thing where you go into a prompt state
            // where it wait till the user answers and class the call back on
            // a yes
            Key::Char('D') | Key::Backspace => {
                let index = self.screen.get_selected_index();
                let tab = self.screen.get_selected_tab();

                let name = state.set(tab).get(index).unwrap().name.clone();

                let delete = move |state: &mut State| {
                    state.set_mut(tab).remove(index);
                };
                self.mode = Mode::Prompt {
                    msg: format!("Remove audio \"{}\"? (y/N)", name),
                    callback: Rc::new(delete),
                };
            }

            Key::Ctrl('j') => {
                let index = self.screen.get_selected_index();
                let tab = self.screen.get_selected_tab();

                let set = state.set_mut(tab);

                if index > 0 && index < set.len() - 1 {
                    set.swap(index, index + 1)
                }
                self.screen.move_down(set.len());
            }
            Key::Ctrl('k') => {
                let index = self.screen.get_selected_index();
                let tab = self.screen.get_selected_tab();

                let set = state.set_mut(tab);

                if index < set.len() - 1 {
                    set.swap(index, index - 1)
                }

                self.screen.move_up();
            }

            Key::Char('1') => self.screen.select_tab(0),
            Key::Char('2') => self.screen.select_tab(1),

            // Key::Tab => self.screens.next_tab(),
            Key::Tab => todo!(),
            Key::Alt('e') => todo!("enter search"),
            _ => todo!(),
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.deinit().unwrap();
    }
}
