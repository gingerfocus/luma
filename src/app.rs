use std::rc::Rc;
use tui::{prelude::*, widgets::Clear};

use crate::{
    event::{Key, Mouse},
    mode::{InsertData, PromptData},
    prelude::*,
    ui::screen::Screen,
};

pub struct App {
    pub run: bool,
    screen: Screen,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    // Will always be some when in prompt mode
    // prompt_callback: Option<Box<dyn Fn(&mut State) -> ()>>,

    // Will always be some when in prompt mode
    // insert_buffer: Option<String>,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

        Ok(Self {
            run: true,
            screen: Screen::default(),
            terminal,
        })
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        let Rect { width, height, .. } = self.terminal.size()?;
        self.screen.configure_surface(width, height);
        self.screen.init()?;

        self.run = true;

        Ok(())
    }

    pub fn draw(&mut self, luma: &Luma, mode: &Mode) -> Result<()> {
        self.terminal.draw(|f| {
            // configure the surface so out drawing boxes are the right size
            // if they changed since last render.
            let Rect { width, height, .. } = f.size();
            self.screen.configure_surface(width, height);

            let tab = self.screen.get_selected_tab_index();
            let index = self.screen.get_selected_index();
            let set = luma
                .get_set_by_index(tab)
                .expect("A Valid tab is not selected");

            if let Some(link) = index.and_then(|i| set.get(i)) {
                // this fixes a bug where when you delete the last entry you are hovering nothing
                // if index > set.len().saturating_sub(1) {
                //     self.screen.select_index(set.len() - 1)
                // }

                let preview = crate::ui::render::preview(link);
                f.render_widget(preview, self.screen.preview_pane);
            }

            let list = crate::ui::render::list(set);
            f.render_stateful_widget(list, self.screen.side_pane, self.screen.list_state_mut());

            let names = luma.get_set_names();
            let tabs = crate::ui::render::tabs(names, tab);
            f.render_widget(tabs, self.screen.title_bar);

            match mode {
                Mode::Normal => {}
                Mode::Prompt(data) => {
                    let prompt_render = crate::ui::render::prompt(&data.prompt);
                    let float_box = crate::ui::render::float_box(&data.prompt, width, height);

                    f.render_widget(Clear, float_box);
                    f.render_widget(prompt_render, float_box);
                }
                Mode::Insert(data) => {
                    let prompt = data.prompts.get(data.index).unwrap();
                    let buffer = data.buffers.get(data.index).unwrap();
                    let msg = format!("{}: {}", prompt, buffer);
                    let paragraph = crate::ui::render::input(&msg);

                    let new_width = std::cmp::max(msg.len() as u16 + 2, width - 20);
                    let new_height = 3;
                    let x = (width - new_width) / 2;
                    let y = (height - new_height) / 2;

                    let render_box = Rect {
                        x,
                        y,
                        width: new_width,
                        height: new_height,
                    };

                    let clear_box = Rect {
                        x: x - 2,
                        y,
                        width: new_width + 1,
                        height: new_height + 1,
                    };

                    f.render_widget(Clear, clear_box);
                    f.render_widget(paragraph, render_box);
                }
            }
        })?;

        Ok(())
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
        mode: &mut Mode,
    ) -> anyhow::Result<LumaMessage> {
        match event {
            Event::GainedFocus(_did) => anyhow::bail!("not implemented"),
            Event::Input(key) => self.handle_key(key, luma, mode),
            Event::Click(click) => Ok(self.handle_click(click, luma, mode)),
            Event::Paste(paste) => Ok(self.handle_paste(&paste, luma, mode)),
            Event::Resize(_x, _y) => Ok(LumaMessage::Nothing),
            Event::Tick => Ok(LumaMessage::Nothing),
        }
    }

    fn handle_paste(&self, paste: &str, _luma: &mut Luma, mode: &mut Mode) -> LumaMessage {
        match mode {
            Mode::Normal => LumaMessage::Nothing,
            Mode::Prompt { .. } => LumaMessage::Nothing,
            Mode::Insert(data) => {
                data.buffers
                    .get_mut(data.index)
                    .expect("something has gone very wrong for you to get here")
                    .push_str(paste);
                LumaMessage::Redraw
            }
        }
    }

    #[allow(unused, clippy::needless_pass_by_ref_mut)]
    fn handle_click(&mut self, click: Mouse, luma: &mut Luma, _mode: &mut Mode) -> LumaMessage {
        let Mouse { kind, .. } = click;
        match kind {
            crate::event::MouseKind::LeftClick => todo!(),
            crate::event::MouseKind::RightClick => todo!(),
            crate::event::MouseKind::MiddleClick => todo!(),
            crate::event::MouseKind::Drag => todo!(),
            crate::event::MouseKind::Scroll(i) => {
                // TODO: make sure the cursor is over the thing
                self.screen.move_cursor(i);
                LumaMessage::Redraw
            }
            crate::event::MouseKind::Nothing => LumaMessage::Nothing,
        }
    }

    fn handle_key(
        &mut self,
        key: Key,
        luma: &mut Luma,
        mode: &mut Mode,
    ) -> anyhow::Result<LumaMessage> {
        match mode {
            Mode::Normal => Ok(self.handle_key_normal(key, luma)),
            Mode::Prompt(data) => Ok(self.handle_key_prompt(key, luma, data)),
            Mode::Insert(data) => self.handle_key_insert(key, luma, data),
        }

        // if let Some(m) = res {
        //     *mode = m;
        // }
        // LumaMessage::Nothing;
    }

    fn handle_key_prompt(&mut self, key: Key, state: &mut Luma, data: &PromptData) -> LumaMessage {
        match key {
            // the accept keys
            Key::Char('y') | Key::Char('Y') => {
                (data.accepted)(state);
                LumaMessage::SetMode(Mode::Normal)
            }
            // the decline keys
            Key::Enter | Key::Char('n') | Key::Char('N') => {
                (data.declined)(state);
                LumaMessage::SetMode(Mode::Normal)
            }
            // the cancel keys
            Key::Esc | Key::Char('q') | Key::Ctrl('c') => LumaMessage::SetMode(Mode::Normal),
            // everything else is ignored
            _ => LumaMessage::Nothing,
        }
    }

    fn handle_key_insert(
        &mut self,
        key: Key,
        state: &mut Luma,
        data: &mut InsertData,
    ) -> anyhow::Result<LumaMessage> {
        match key {
            Key::Esc | Key::Ctrl('c') => Ok(LumaMessage::SetMode(Mode::Normal)),
            Key::Char(ch) => {
                data.buffers
                    .get_mut(data.index)
                    .ok_or(anyhow::anyhow!("failed to index buffers"))?
                    .push(ch);

                Ok(LumaMessage::Redraw)
            }
            Key::Enter => match data.next_or_destructure() {
                Some((buffers, callback)) => {
                    callback(state, buffers);
                    Ok(LumaMessage::SetMode(Mode::Normal))
                }
                None => Ok(LumaMessage::Redraw),
            },
            Key::Backspace => {
                data.buffers
                    .get_mut(data.index)
                    .ok_or(anyhow::anyhow!("failed to index buffers"))?
                    .pop();
                Ok(LumaMessage::Redraw)
            }
            _ => Ok(LumaMessage::Nothing),
        }
    }

    fn handle_key_normal(&mut self, key: Key, luma: &mut Luma) -> LumaMessage {
        match key {
            Key::Char('q') | Key::Ctrl('c') => {
                self.run = false;
                LumaMessage::Exit
            }
            Key::Esc => LumaMessage::Nothing,
            Key::Left | Key::Char('h') => todo!(),
            Key::Down | Key::Char('j') => {
                self.screen.move_cursor(1);
                LumaMessage::Redraw
            }
            Key::Up | Key::Char('k') => {
                self.screen.move_cursor(-1);
                LumaMessage::Redraw
            }
            Key::Right | Key::Char('l') => LumaMessage::Nothing,

            Key::Char('i') => {
                let tab = self.screen.get_selected_tab_index();
                let o_index = self.screen.get_selected_index();

                let callback = Rc::new(move |luma: &mut Luma, buffers: Vec<String>| {
                    let link = &buffers[0];
                    let name = &buffers[1];
                    luma.insert_at_index(tab, o_index, Link::new(name, link));
                });

                let prompts = ["link".into(), "name".into(), "".into(), "".into()];
                LumaMessage::SetMode(Mode::Insert(InsertData::new(prompts, callback).unwrap()))
            }

            Key::Enter => {
                let tab = self.screen.get_selected_tab_index();
                let set = luma.get_set_by_index(tab).expect("Set should not be nil");
                if let Some(index) = self.screen.get_selected_index() {
                    if let Some(link) = set.get(index) {
                        let opener = luma.get_opener("audio");
                        if let Some(o) = opener {
                            o.open(&link.link);
                        }
                    }
                }

                // if anything this should be the open request
                LumaMessage::Nothing
            }

            Key::Char('g') => {
                self.screen.select_index(0);
                LumaMessage::Redraw
            }
            Key::Char('G') => {
                let tab = self.screen.get_selected_tab_index();
                let len = luma.get_set_len_by_index(tab);
                self.screen.select_index(len - 1);
                LumaMessage::Redraw
            }

            Key::Char('?') => {
                let prompt = "\
j k     -       Move Up/Down\n\
g G     -       Go Top/Bottom\n\
Enter   -       Open Link in Browser\n\
c-j c-k -       Move Link Up/Down\n\
i       -       Add Link\n\
D       -       Delete entry\n\
q       -       Exit\
                "
                .to_string()
                .into_boxed_str();
                let nothing = |_luma: &mut Luma| {};

                LumaMessage::SetMode(Mode::Prompt(crate::mode::PromptData {
                    prompt,
                    accepted: Rc::new(nothing),
                    declined: Rc::new(nothing),
                }))
            }

            // try creating some sort of thing where you go into a prompt state
            // where it wait till the user answers and class the call back on
            // a yes
            Key::Char('D') | Key::Backspace => {
                // if nothing is hovered then just do nothing
                let tab = self.screen.get_selected_tab_index();
                let set = luma.get_set_by_index(tab).expect("dont faul");

                if let Some(index) = self.screen.get_selected_index() {
                    let link = set.get(index).expect("waaaaaaaa");
                    let name = link.name.clone();

                    let delete = move |luma: &mut Luma| {
                        luma.get_mut_set_by_index(tab).unwrap().remove(index);
                    };
                    let nothing = |_luma: &mut Luma| {};

                    LumaMessage::SetMode(Mode::Prompt(crate::mode::PromptData {
                        prompt: format!("Remove audio \"{}\"? (y/N)", name).into_boxed_str(),
                        accepted: Rc::new(delete),
                        declined: Rc::new(nothing),
                    }))
                } else {
                    LumaMessage::Nothing
                }
            }

            Key::Ctrl('j') => {
                let index = match self.screen.get_selected_index() {
                    Some(i) => i,
                    None => return LumaMessage::Nothing,
                };

                let tab = self.screen.get_selected_tab_index();
                let set = match luma.get_mut_set_by_index(tab) {
                    Some(set) => set,
                    None => return LumaMessage::Nothing,
                };

                if index > 0 && index < set.len() - 1 {
                    set.swap(index, index + 1)
                }
                self.screen.move_cursor(1);
                LumaMessage::Redraw
            }
            Key::Ctrl('k') => {
                let index = match self.screen.get_selected_index() {
                    Some(i) => i,
                    None => return LumaMessage::Nothing,
                };
                let tab = self.screen.get_selected_tab_index();
                let set = luma.get_mut_set_by_index(tab).expect("Set must exists");

                if index < set.len() - 1 {
                    set.swap(index, index - 1)
                }

                // self.screen.move_up();
                LumaMessage::Redraw
            }

            Key::Char('1') => numeric_key_option!(self, luma, 0),
            Key::Char('2') => numeric_key_option!(self, luma, 1),
            Key::Char('3') => numeric_key_option!(self, luma, 2),
            Key::Char('4') => numeric_key_option!(self, luma, 3),

            // Key::Tab => self.screens.next_tab(),
            Key::Tab => todo!(),
            Key::Alt('e') => todo!("enter search"),
            _ => todo!(),
        }
    }
}
use crate::numeric_key_option;

#[macro_export]
macro_rules! numeric_key_option {
    ( $self:ident, $luma:ident, $x:expr ) => {{
        let name = match $luma.get_key_by_index($x) {
            Some(n) => n,
            None => return LumaMessage::Nothing,
        };

        let ind = $crate::ui::screen::IndexedPair { index: $x, name };
        $self.screen.select_tab(ind);
        LumaMessage::Redraw
    }};
}

impl Drop for App {
    fn drop(&mut self) {
        self.deinit().unwrap();
    }
}
