use tui::{
    prelude::*,
    widgets::{Clear, ListState},
};

use crate::prelude::*;

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
    pub fn new() -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

        Ok(Self {
            run: true,
            screen: Default::default(),
            terminal,
        })
    }

    pub fn init(&mut self) -> Result<()> {
        self.run = true;

        self.screen.init()?;

        let Rect { width, height, .. } = self.terminal.size()?;
        self.screen.configure_surface(width, height);

        Ok(())
    }

    pub fn deinit(&mut self) -> Result<()> {
        self.run = false;

        self.screen.deinit()?;

        Ok(())
    }

    pub fn redraw(&mut self, mode: &Mode) -> Result<()> {
        self.terminal.draw(|f| f.render_widget(Clear, f.size()))?;
        self.draw(mode)
    }

    pub fn draw(&mut self, mode: &Mode) -> Result<()> {
        log::debug!("redraw started");
        self.terminal.draw(|f| {
            // configure the surface so out drawing boxes are the right size
            // if they changed since last render.
            let Rect { width, height, .. } = f.size();
            self.screen.configure_surface(width, height);

            let (tab, index) = futures::executor::block_on(util::get_tab_and_index());
            log::trace!("selected tab is: {}", index);
            log::trace!("selected index is: {}", index);

            {
                let luma = futures::executor::block_on(async { LUMA.read().await });

                let set = luma.get_index(tab).expect("A valid tab is not selected");

                if let Some(link) = set.1.get(index) {
                    // TODO: when deleting the last item the cursor hovers nothing

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
            }

            match mode {
                Mode::Normal => {}
                Mode::Prompt(data) => {
                    let prompt_render = crate::ui::render::prompt(&data.prompt);
                    let float_box = crate::ui::render::float_box(&data.prompt, width, height);

                    f.render_widget(Clear, float_box);
                    f.render_widget(prompt_render, float_box);
                }
                Mode::Insert(data) => {
                    // let Some((prompt, buffer, _resp)) = data.last() else {
                    //     log::warn!("render tried to draw and out of data prompt buffer");
                    //     return;
                    // };
                    if let Some((prompt, buffer, _resp)) = data.last() {
                        let msg = format!("{}{}", prompt, buffer);
                        let msglen = msg.len() as u16;
                        let paragraph = crate::ui::render::input(&msg);

                        let new_width = std::cmp::max(msglen + 2, width - 20);
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
                        f.set_cursor(render_box.x + 1 + msglen, render_box.y + 1);
                    } else {
                        log::warn!("render tried to draw and out of date prompt buffer");
                    }
                }
            }
        })?;

        log::debug!("redraw done");

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.deinit().unwrap();
    }
}
