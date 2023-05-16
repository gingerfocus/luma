// use anyhow::Result;
// use crossterm::terminal::{
//     disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
// };
// use std::io::{self, Stdout};
// use tui::{
//     backend::CrosstermBackend,
//     layout::{Constraint, Direction, Layout, Rect},
//     style::{Modifier, Style},
//     text::{Span, Spans, Text},
//     widgets::{Block, Borders, Paragraph},
//     // layout::{Constraint, Direction, Layout},
//     // widgets::{Block, Borders, Paragraph},
//     // Frame,
//     Terminal,
// };

use crate::state::{Section, Link};

pub enum Line<'a> {
    Header(&'a mut Section),
    Link(&'a Link),
    Note,
}

impl Line<'_> {
    pub fn display(&self) -> String {
        match self {
            Line::Header(s) => s.header_format(),
            Line::Link(l) => l.link_format(),
            Line::Note => { todo!() },
        }
    }
}

// pub enum Updater {
//     Full,
//     Main,
//     Status,
//     None,
// }
//
// struct Screen {
//     main_box: Rect,
//     status_line: Rect,
//     term: Terminal<CrosstermBackend<Stdout>>,
//     last_window_size: (u16, u16),
// }
//
// impl Screen {
//     fn new() -> Result<Self> {
//         // setup terminal
//         enable_raw_mode()?;
//         let mut stdout = io::stdout();
//         crossterm::execute!(stdout, EnterAlternateScreen)?;
//         let backend = CrosstermBackend::new(stdout);
//         let mut term = Terminal::new(backend)?;
//
//         term.clear()?;
//
//         let Rect { _, _, width, height } = term.size()?;
//         let main_box = Rect::new(0, 0, size.width, size.height - 1);
//         let status_line = Rect::new(0, size.height - 1, size.width, size.height);
//
//         Ok(Screen {
//             main_box,
//             status_line,
//             term,
//             last_window_size: size,
//         })
//     }
//
//     /*
//     * ┌──────────────��──────────────┐
//     *
//     *)
//                    .direction(Direction::Vertical)
//                    // .margin(1)
//                    .constraints(
//                        [
//                            Constraint::Length(1), // status bar
//                            Constraint::Min(1), // the rest
//                        ]
//                        .as_ref(),
//                    )
//                    .split(f.size());
//
//                let status_line = chunks[0];
//
//                let div = Layout::default()
//                    .direction(Direction::Vertical)
//                    .constraints(
//                        [
//                            Constraint::Percentage(20),
//                            Constraint::Min(6),
//                            Constraint::Percentage(20),
//                        ]
//                        .as_ref(),
//                    )
//                    .split(chunks[1]);
//
//                let main_box: Rect = Layout::default()
//                    .direction(Direction::Horizontal)
//                    .constraints([
//                        Constraint::Percentage(20),
//                        // Constraint::Percentage(60),
//                        Constraint::Min(6),
//                        Constraint::Percentage(20),
//                        ].as_ref()
//                    )
//                    .split(div[1])[1];
//
//
//     * ┌──────────────────────────────┐
//     * */
//
//     fn draw_main(&mut self) -> Result<()> { 
//         // let desktop = Paragraph::new("this is where the current boot os would go");
//         // f.render_widget(desktop, chunks[1]);
//
//         // let name = Block::default().title("Hostname").borders(Borders::ALL);
//         // f.render_widget(name, main_box);
//
//         // log.write("Wrote box".as_bytes()).unwrap();
//
//         // let pass = Block::default().title("Password").borders(Borders::ALL);
//         // f.render_widget(pass, chunks[3]);
//
//         let desktop_text = vec![
//             Spans::from(vec![
//                 Span::raw("< "),
//                 Span::styled("Hostname", Style::default().add_modifier(Modifier::BOLD)),
//                 Span::raw(" >"),
//             ]),
//             Spans::from(vec![
//                 Span::raw("Username: "),
//                 Span::styled("root", Style::default().add_modifier(Modifier::BOLD)),
//             ]),
//             Spans::from(vec![
//                 Span::raw("Password: "),
//                 Span::styled("********", Style::default().add_modifier(Modifier::BOLD)),
//             ]),
//         ];
//
//         let desktop = Paragraph::new(desktop_text)
//             .block(Block::default().title("Hostname").borders(Borders::ALL));
//         // .alignment(Alignment::Center);
//
//         self.term.draw(|f|  {
//             f.render_widget(desktop, self.main_box);
//         })?;
//
//         Ok(())
//     }
//
//     fn draw_status(&mut self) -> Result<()> {
//         let text = Text::from(Spans::from(vec![
//             Span::raw("Reboot: "),
//             Span::styled("F1", Style::default().add_modifier(Modifier::BOLD)),
//             Span::raw(", Shutdown: "),
//             Span::styled("F2", Style::default().add_modifier(Modifier::BOLD)),
//             Span::raw(", Capslock: todo! "),
//             Span::raw("Renders: "),
//         ]));
//             // .patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));
//
//         let help_message = Paragraph::new(text);
//
//         self.term.draw(|f|  {
//             f.render_widget(help_message, self.status_line);
//         })?;
//
//         Ok(())
//     }
//
//     // TODO this should get the currently displayed text from the main thread
//     pub fn draw(&mut self, update_plan: Updater) -> Result<()> {
//
//         // NOTE this is wrong as the first two elements are the x and y
//         let Rect { width, height, .. } = self.term.size()?;
//
//         if (width, height) != self.last_window_size {
//             self.last_window_size = (width, height);
//             self.main_box = Rect::new(0, 0, width, height - 1);
//             self.status_line = Rect::new(0, height - 1, width, 1);
//         }
//
//         match update_plan {
//             Updater::Full => {
//                 self.draw_main()?;
//                 self.draw_status()?;
//             }
//             Updater::Main => {
//                 self.draw_main()?;
//             }
//             Updater::Status => {
//                 self.draw_status()?;
//             }
//             Updater::None => {}
//         }
//
//         self.term.draw(|f| {
//
//             // f.set_cursor();
//         })?;
//
//         //     let input = Paragraph::new(app.input.as_ref())
//         //         .style(InputMode::Editing => Style::default().fg(Color::Yellow))
//         //         .block(Block::default().borders(Borders::ALL).title("Input"));
//         //
//         //
//         //
//         //     let messages: Vec<ListItem> = app
//         //         .messages
//         //         .iter()
//         //         .enumerate()
//         //         .map(|(i, m)| {
//         //             let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
//         //             ListItem::new(content)
//         //         })
//         //         .collect();
//         //     let messages = List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
//         //     f.render_widget(messages, chunks[2]);
//         // }
//
//         Ok(())
//     }
//
//     pub fn close(&mut self) -> Result<(), std::io::Error> {
//         // restore terminal
//         disable_raw_mode()?;
//         crossterm::execute!(self.term.backend_mut(), LeaveAlternateScreen,)?;
//         self.term.show_cursor()?;
//         Ok(())
//     }
// }

