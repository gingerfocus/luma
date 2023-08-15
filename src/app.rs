use std::{cell::RefCell, process::Stdio, rc::Rc};

use crate::{event::Key, luma::Link, prelude::*, ui::Screen};

#[derive(Default)]
pub struct App {
    pub run: bool,
    screen: Screen,
    // Will always be some when in prompt mode
    // prompt_callback: Option<Box<dyn Fn(&mut State) -> ()>>,

    // Will always be some when in prompt mode
    // insert_buffer: Option<String>,
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

    pub fn draw(&mut self, luma: &Luma, mode: &Mode) {
        self.screen.draw(luma, mode);
    }

    pub fn deinit(&mut self) -> anyhow::Result<()> {
        self.screen.deinit()?;

        self.run = false;

        Ok(())
    }
}

impl App {
    pub fn handle(
        &mut self,
        event: Event,
        luma: &mut Luma,
        mode: &Mode,
    ) -> anyhow::Result<LumaMessage> {
        match event {
            Event::GainedFocus(_did) => anyhow::bail!("not implemented"),
            Event::Input(key) => Ok(self.handle_key(key, luma, mode)),
            Event::Click(_me) => anyhow::bail!("not implemented"),
            Event::Paste(paste) => Ok(self.handle_paste(mode, &paste)),
            Event::Resize(_x, _y) => anyhow::bail!("not implemented"),
            Event::Tick => Ok(LumaMessage::Nothing),
        }
    }

    fn handle_paste(&self, mode: &Mode, paste: &str) -> LumaMessage {
        match mode {
            Mode::Normal => LumaMessage::Nothing,
            Mode::Prompt { .. } => LumaMessage::Nothing,
            Mode::Insert { index, buffers, .. } => {
                buffers
                    .borrow_mut()
                    .get_mut(*index.borrow())
                    .unwrap()
                    .push_str(paste);
                LumaMessage::Redraw
            }
        }
    }

    fn handle_key(&mut self, key: Key, luma: &mut Luma, mode: &Mode) -> LumaMessage {
        match mode {
            Mode::Normal => self.handle_key_normal(key, luma),
            Mode::Prompt { callback, .. } => self.handle_key_prompt(key, luma, callback.clone()),
            Mode::Insert {
                index,
                buffers,
                callback,
                ..
            } => {
                self.handle_key_insert(key, luma, index.clone(), buffers.clone(), callback.clone())
            }
        }
    }

    fn handle_key_prompt(
        &mut self,
        key: Key,
        state: &mut Luma,
        callback: Rc<dyn Fn(&mut Luma)>,
    ) -> LumaMessage {
        match key {
            Key::Char('y') | Key::Char('Y') => {
                // This should never panic as this should only ever be able to
                // be ran when it exists
                callback(state);
                LumaMessage::SetMode(Mode::Normal)
            }

            Key::Enter | Key::Char('n') | Key::Char('N') | Key::Esc | Key::Ctrl('c') => {
                LumaMessage::SetMode(Mode::Normal)
            }
            _ => LumaMessage::Nothing,
        }
    }

    #[allow(clippy::type_complexity)]
    fn handle_key_insert(
        &mut self,
        key: Key,
        state: &mut Luma,
        index: Rc<RefCell<usize>>,
        buffers: Rc<RefCell<Vec<String>>>,
        callback: Rc<dyn Fn(&mut Luma, Vec<String>)>,
    ) -> LumaMessage {
        match key {
            Key::Esc | Key::Ctrl('c') => LumaMessage::SetMode(Mode::Normal),
            Key::Char(ch) => {
                buffers
                    .borrow_mut()
                    .get_mut(*index.borrow())
                    .unwrap()
                    .push(ch);
                LumaMessage::Redraw
            }
            Key::Enter => {
                let len = buffers.borrow().len();
                if *index.borrow() < len - 1 {
                    *index.borrow_mut() += 1;
                    LumaMessage::Redraw
                } else {
                    let buffers = buffers.take();
                    callback(state, buffers);
                    LumaMessage::SetMode(Mode::Normal)
                }
            }
            Key::Backspace => {
                buffers.borrow_mut().get_mut(*index.borrow()).unwrap().pop();
                LumaMessage::Redraw
            }
            _ => LumaMessage::Nothing,
        }
    }

    fn handle_key_normal(&mut self, key: Key, state: &mut Luma) -> LumaMessage {
        match key {
            Key::Char('q') | Key::Ctrl('c') => {
                self.run = false;
                LumaMessage::Exit
            }
            Key::Esc => LumaMessage::Nothing,
            Key::Left | Key::Char('h') => todo!(),
            Key::Down | Key::Char('j') => {
                let tab = self.screen.get_selected_tab();
                let len = state.set_mut(tab).len();
                self.screen.move_down(len);
                LumaMessage::Redraw
            }
            Key::Up | Key::Char('k') => {
                self.screen.move_up();
                LumaMessage::Redraw
            }
            Key::Right | Key::Char('l') => todo!(),

            Key::Char('i') => {
                let index = self.screen.get_selected_index();
                let tab = self.screen.get_selected_tab();

                let callback = move |state: &mut Luma, mut buffers: Vec<String>| {
                    let name = buffers.pop().unwrap();
                    let link = buffers.pop().unwrap();

                    state.set_mut(tab).insert(index, Link::new(name, link));
                };

                LumaMessage::SetMode(Mode::Insert {
                    index: Rc::new(RefCell::new(0)),
                    prompts: Rc::new(vec!["link".into(), "name".into()]),
                    buffers: Rc::new(RefCell::new(vec![String::new(), String::new()])),
                    callback: Rc::new(callback),
                })
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

                // if anything this should be the open request
                LumaMessage::Nothing
            }

            Key::Char('g') => {
                self.screen.select_index(0);
                LumaMessage::Redraw
            }
            Key::Char('G') => {
                let tab = self.screen.get_selected_tab();
                let len = state.set_mut(tab).len();
                self.screen.select_index(len - 1);
                LumaMessage::Redraw
            }

            // try creating some sort of thing where you go into a prompt state
            // where it wait till the user answers and class the call back on
            // a yes
            Key::Char('D') | Key::Backspace => {
                let index = self.screen.get_selected_index();
                let tab = self.screen.get_selected_tab();

                let name = state.set(tab).get(index).unwrap().name.clone();

                let delete = move |state: &mut Luma| {
                    state.set_mut(tab).remove(index);
                };
                LumaMessage::SetMode(Mode::Prompt {
                    msg: format!("Remove audio \"{}\"? (y/N)", name),
                    callback: Rc::new(delete),
                })
            }

            Key::Ctrl('j') => {
                let index = self.screen.get_selected_index();
                let tab = self.screen.get_selected_tab();

                let set = state.set_mut(tab);

                if index > 0 && index < set.len() - 1 {
                    set.swap(index, index + 1)
                }
                self.screen.move_down(set.len());
                LumaMessage::Redraw
            }
            Key::Ctrl('k') => {
                let index = self.screen.get_selected_index();
                let tab = self.screen.get_selected_tab();

                let set = state.set_mut(tab);

                if index < set.len() - 1 {
                    set.swap(index, index - 1)
                }

                // self.screen.move_up();
                LumaMessage::Redraw
            }

            Key::Char('1') => {
                self.screen.select_tab(0);
                LumaMessage::Redraw
            }
            Key::Char('2') => {
                self.screen.select_tab(1);
                LumaMessage::Redraw
            }

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
