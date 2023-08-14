use std::io;

use tui::prelude::*;
use tui::widgets::*;

use crate::state::State;

pub struct Screen {
    pub is_valid: bool,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,

    configured_size: (u16, u16),
    title_bar: Rect,
    side_pane: Rect,
    preview_pane: Rect,

    active: StatefulState,
}

#[derive(Default)]
struct StatefulState {
    pub list: ListState,
    pub list_len: usize,
    pub tab: usize,
}

impl Default for Screen {
    fn default() -> Self {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();

        Self {
            is_valid: Default::default(),
            terminal,
            configured_size: Default::default(),
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

    pub fn draw(&mut self, state: &State) {
        // some work before drawing
        let Rect { width, height, .. } = self.terminal.size().unwrap();
        self.configure_surface(width, height);

        // updateing state as nessisary
        self.active.list_len = state.audios.len();

        self.terminal
            .draw(|f| {
                let current_set = match self.active.tab {
                    0 => &state.audios,
                    1 => &state.videos,
                    i => panic!("Note a falid tab: {}", i),
                };
                let list = List::new(
                    current_set
                        .iter()
                        .map(|l| ListItem::new(Line::from(vec![Span::raw(l.name.clone())])))
                        .collect::<Vec<ListItem>>(),
                )
                // .highlight_symbol(">")
                .highlight_style(Style::default().bg(Color::Red))
                .block(Block::default().title("links").borders(Borders::all()));

                f.render_stateful_widget(list, self.side_pane, &mut self.active.list);

                let preview = self
                    .active
                    .list
                    .selected()
                    .map(|i| state.audios.get(i))
                    .flatten()
                    .map(|r| r.as_paragraph())
                    .unwrap_or(Paragraph::new(""))
                    .block(Block::default().title("Info").borders(Borders::all()));

                f.render_widget(preview, self.preview_pane);

                let titles: Vec<Line<'_>> = ["Audios", "Videos"]
                    .iter()
                    .cloned()
                    .map(Line::from)
                    .collect();

                let tabs = Tabs::new(titles)
                    .select(self.active.tab)
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::Yellow))
                    .divider(symbols::DOT);

                f.render_widget(tabs, self.title_bar);
            })
            .unwrap();
    }

    pub fn move_down(&mut self) {
        let index = self.active.list.selected().unwrap_or(0);
        if index < self.active.list_len - 1 {
            self.active.list.select(Some(index + 1))
        }
    }

    pub fn move_up(&mut self) {
        let index = self.active.list.selected().unwrap_or(self.active.list_len);
        if index > 0 {
            self.active.list.select(Some(index - 1))
        }
    }

    pub fn selecte_tab(&mut self, index: usize) {
        // TODO: Bounds checks
        self.active.tab = index;
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

// pub fn draw(&self, f: &mut Frame<'_, CrosstermBackend<Stdout>>) {
// }
//
