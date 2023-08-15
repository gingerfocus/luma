use std::io;

use super::render::render_input;
use super::render::render_prompt;
use super::traits::AsListItem;
use super::traits::AsParagraph;

use crate::prelude::*;
use tui::prelude::*;
use tui::widgets::*;

use crate::luma::Link;

pub struct Screen {
    pub is_valid: bool,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    configured_size: (u16, u16),

    title_bar: Rect,
    side_pane: Rect,
    preview_pane: Rect,

    active: ScreenState,
}

#[derive(Default)]
struct ScreenState {
    list: ListState,
    tab: usize,
}

impl Default for Screen {
    fn default() -> Self {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();

        Self {
            is_valid: Default::default(),
            terminal,
            configured_size: (0, 0),
            title_bar: Default::default(),
            side_pane: Default::default(),
            preview_pane: Default::default(),
            active: Default::default(),
        }
    }
}

impl Screen {
    pub fn init(&mut self) -> anyhow::Result<()> {
        if !self.is_valid {
            crossterm::terminal::enable_raw_mode().unwrap();
            crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
            self.terminal.clear().unwrap();

            self.active.list.select(Some(0));

            let Rect { width, height, .. } = self.terminal.size()?;
            self.configure_surface(width, height);

            self.is_valid = true;
        }
        Ok(())
    }

    fn configure_surface(&mut self, x: u16, y: u16) {
        if self.configured_size != (x, y) {
            self.configured_size = (x, y);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
                .split(Rect::new(0, 0, x, y));

            self.title_bar = chunks[0];

            let div = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Min(1)].as_ref())
                .split(chunks[1]);

            self.side_pane = div[0];
            self.preview_pane = div[1];
        }
    }

    pub fn draw(&mut self, state: &Luma, mode: &Mode) {
        // some work before drawing
        let Rect { width, height, .. } = self.terminal.size().unwrap();
        self.configure_surface(width, height);

        let tab = self.get_selected_tab();
        let set = state.set(tab);

        // this fixes a bug where when you delete the last entry you are hovering nothing
        let index = self.get_selected_index();
        if index > set.len().saturating_sub(1) {
            self.select_index(set.len() - 1)
        }

        let list = List::new(
            set.iter()
                .map(AsListItem::as_list_item)
                .collect::<Vec<ListItem>>(),
        )
        // .highlight_symbol(">")
        .highlight_style(Style::default().bg(Color::Red))
        .block(Block::default().title("links").borders(Borders::all()));

        let preview = self
            .active
            .list
            .selected()
            .and_then(|i| set.get(i))
            .map(AsParagraph::as_paragraph)
            .unwrap_or(Paragraph::new(""))
            .block(Block::default().title("Info").borders(Borders::all()));

        let tabs = Tabs::new(
            ["Audios", "Reading"]
                .iter()
                .cloned()
                .map(Line::from)
                .collect::<Vec<Line<'_>>>(),
        )
        .select(self.active.tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(symbols::DOT);

        self.terminal
            .draw(|f| {
                f.render_stateful_widget(list, self.side_pane, &mut self.active.list);

                f.render_widget(preview, self.preview_pane);

                f.render_widget(tabs, self.title_bar);

                match mode {
                    Mode::Normal => {}
                    Mode::Prompt { msg, .. } => render_prompt(f, msg),
                    Mode::Insert {
                        prompts,
                        buffers,
                        index,
                        ..
                    } => {
                        let prompt = prompts.get(*index.borrow()).unwrap();
                        let buffers = buffers.borrow();
                        let buffer = buffers.get(*index.borrow()).unwrap();
                        render_input(f, format!("{}: {}", prompt, buffer).as_str())
                    }
                }
            })
            .unwrap();
    }

    pub fn move_down(&mut self, max: usize) {
        let index = self.active.list.selected().unwrap_or(0);
        if index < max - 1 {
            self.active.list.select(Some(index + 1));
        }
    }

    pub fn move_up(&mut self) {
        let index = self.active.list.selected().unwrap_or(0);
        if index > 0 {
            self.active.list.select(Some(index - 1));
        }
    }

    pub fn select_tab(&mut self, index: usize) {
        // TODO: Bounds checks
        self.active.tab = index;
        self.active.list.select(Some(0));
    }

    pub fn get_selected_tab(&mut self) -> usize {
        self.active.tab
    }

    pub fn select_index(&mut self, index: usize) {
        // TODO: Bounds checks
        self.active.list.select(Some(index));
    }

    pub fn get_selected_index(&self) -> usize {
        self.active.list.selected().unwrap_or(0)
    }

    pub fn choose_set<'a>(&self, state: &'a Luma) -> &'a Vec<Link> {
        match self.active.tab {
            0 => &state.audios,
            1 => &state.reading,
            i => panic!("Note a falid tab: {}", i),
        }
    }

    pub fn deinit(&mut self) -> anyhow::Result<()> {
        if self.is_valid {
            // restore terminal, results are ignored beacuse we are leaving anyway
            crossterm::terminal::disable_raw_mode()?;
            crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
            _ = self.terminal.show_cursor();
            // crossterm::execute!(stdout, Clear(ClearType::All))?;
            self.is_valid = false;
        }
        Ok(())
    }
}

mod test {

    #[test]
    fn check_index_overflows() {
        let mut screen = super::Screen::default();

        screen.select_index(8);
        screen.move_down(8);

        assert!(screen.get_selected_index() == 8);

        screen.select_index(0);
        screen.move_up();

        assert!(screen.get_selected_index() == 0);
    }
}
