use tui::{
    prelude::*,
    widgets::{Clear, ListState},
};

use crate::{prelude::*, ui::screen::Screen};

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

            let gread = STATE.read().unwrap();
            let tab = gread.selected_tab;
            let index = gread.selected_index;
            let set = luma.get_index(tab).expect("A valid tab is not selected");

            if let Some(link) = set.1.get(index) {
                // this fixes a bug where when you delete the last entry you are hovering nothing
                // if index > set.len().saturating_sub(1) {
                //     self.screen.select_index(set.len() - 1)
                // }

                let preview = crate::ui::render::preview(link);
                f.render_widget(preview, self.screen.preview_pane);
            }

            let list = crate::ui::render::list(set.1);

            let mut state = ListState::default();
            state.select(Some(index));
            f.render_stateful_widget(list, self.screen.side_pane, &mut state);

            let names = luma.keys();
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

impl Drop for App {
    fn drop(&mut self) {
        self.deinit().unwrap();
    }
}
