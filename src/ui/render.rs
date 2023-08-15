use tui::{prelude::*, widgets::*, Frame};

use crate::prelude::*;

pub fn render_prompt(f: &mut Frame<CrosstermBackend<io::Stdout>>, msg: &str) {
    let Rect { width, height, .. } = f.size();
    let new_width = msg.len() as u16 + 2;
    let new_height = 3;

    let x = (width - new_width) / 2;
    let y = (height - new_height) / 2;

    let render_box = Rect {
        x,
        y,
        width: new_width,
        height: new_height,
    };

    let paragraph = Paragraph::new(msg.clone())
        .block(
            Block::default()
                .title("Prompt")
                .borders(Borders::all())
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, render_box);
}

pub fn render_input(f: &mut Frame<CrosstermBackend<io::Stdout>>, msg: &str) {
    let Rect { width, height, .. } = f.size();
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

    let paragraph = Paragraph::new(msg.clone())
        .block(
            Block::default()
                .title("Prompt")
                .borders(Borders::all())
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Left);

    let clear_box = Rect {
        x: x - 2,
        y,
        width: new_width + 1,
        height: new_height + 1,
    };

    f.render_widget(Clear, clear_box);
    f.render_widget(paragraph, render_box);
}

// use std::io;
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

// impl Screen {
//     ┌────────────────────────────┐
//     ┌────────────────────────────┐
//
//     fn draw_main(&mut self) -> Result<()> {
//         // let desktop = Paragraph::new("this is where the current boot os would go");
//         // f.render_widget(desktop, chunks[1]);
//
//         // let name = Block::default().title("Hostname").borders(Borders::ALL);
//         // f.render_widget(name, main_box);
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
