mod render;
mod state;

use crate::{
    render::Line,
    state::{MarkdownFile, Section},
};
use anyhow::Result;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode::*},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rustyline::{history::FileHistory, Editor};
use simplelog::WriteLogger;
use state::Link;

use std::{
    cmp::{max, min},
    fs,
    io::stdout,
    process,
};

fn main() -> Result<()> {
    if std::env::var("LYNKD_LOG").is_ok() {
        WriteLogger::init(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            fs::File::create("lynkd.log")?,
        )?;
    }

    let file = std::env::args().nth(1).unwrap_or_else(|| {
        log::error!("Please provide a file to open");
        eprintln!("Please provide a file to open");
        process::exit(1);
    });

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    let f = fs::File::open(&file)?;
    let mut state = MarkdownFile::from_file(f);

    // create the line reader that is used to get input from the user
    let mut l: Editor<(), FileHistory> = rustyline::Editor::with_config(
        rustyline::config::Builder::new()
            .completion_type(rustyline::config::CompletionType::List)
            .edit_mode(rustyline::config::EditMode::Vi)
            .build(),
    )?;

    let mut index: usize = 0; // the place of the cursor along the screen
    let mut run = true;

    let mut full_lines: Vec<Line> = Vec::new();

    while run {
        execute!(stdout, Clear(ClearType::All)).unwrap();

        let (_, height) = term_size::dimensions().unwrap();

        let starting = max(index as i32 - 5, 0) as usize;

        let mut print = 0;

        let mut active_lines: Vec<&mut Line> = Vec::new();

        full_lines
            .iter_mut()
            .skip(starting)
            .take(height - 1)
            .for_each(|line| {
                execute!(stdout, cursor::MoveTo(0, print as u16)).unwrap();
                print!("{}", line.display());
                // print!("{}: {}", j + starting, entry.title);

                print += 1;

                // if !entry.folded {
                //     for l in entry.notes.iter() {
                //         execute!(stdout, cursor::MoveTo(0, print as u16)).unwrap();
                //         print!("\t{}", l);
                //         active_lines.push(Line::Note);
                //         print += 1;
                //     }
                // }

                active_lines.push(line);
            });

        let cursor_pos = min(index as u16, 5);
        execute!(stdout, cursor::MoveTo(0, cursor_pos)).unwrap();

        // NOTE: this is very hacky and may drop events
        // however it has not yet so until then i will just chill
        let event = read()?;

        // FocusGained,
        // FocusLost,
        // Key(KeyEvent),
        // Mouse(MouseEvent),
        // Paste(String),
        // Resize(u16, u16),

        if let Event::Key(kd) = event {
            match kd.code {
                Esc => { /* cur_mode = Mode::Normal; */ }
                Char('q') => run = false,
                Char('j') | Down => {
                    if index < state.sections.len() - 1 {
                        index += 1
                    }
                }
                Char('k') | Up => index = index.saturating_sub(1),
                Char('g') => index = 0,
                Char('G') => index = state.sections.len() - 1,
                Char('i') => {
                    execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    let title = l.readline("Title: ")?;

                    execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    let url = l.readline("URL: ")?;

                    state
                        .current_section(index)
                        .links
                        .push(Link::new(title, url));
                }
                Enter => {
                    if let Some(Line::Link(lnk)) = active_lines.get(index) {
                        lnk.open()?;
                    }
                }
                Char('m') => {
                    todo!();
                    //     let cur_line = active_lines.get_mut(index).unwrap();
                    //     if let Line::Entry(ent) = cur_line {
                    //         execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    //         if let Ok(name) = l.readline_with_initial("Rename: ", (ent.title.as_str(), "")) {
                    //             ent.title = name.clone();
                    //         }
                    //     }
                }
                Char('f') => {
                    todo!();
                    // let old = entries.get_mut(index).unwrap();
                    // old.folded = !old.folded;
                }
                Char('n') => {
                    todo!();
                    // execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    // let old = entries.get_mut(index).unwrap();
                    //
                    // let note = l.readline("Note: ")?;
                    // old.notes.push(note);
                }
                Char('d') => {
                    todo!();
                    //     execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    //     let r = l.readline("Delete [y/N]: ");
                    //     match r {
                    //         Ok(c) if c == "y" => {
                    //             entries.remove(index);
                    //         }
                    //         _ => {}
                    //     };
                }
                Left | Char('h') => {
                    todo!();
                    //     if index > 0 {
                    //         let entry = entries.remove(index);
                    //         entries.insert(index - 1, entry);
                    //         index -= 1;
                    //     }
                }
                Right | Char('l') => {
                    todo!();
                    //     if index < entries.len() - 1 {
                    //         let entry = entries.remove(index);
                    //         entries.insert(index + 1, entry);
                    //         index += 1;
                    //     }
                }
                _ => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    let out = state
        .sections
        .iter()
        .map(|sec| sec.format())
        .collect::<Vec<String>>()
        .join("\n");

    execute!(stdout, Clear(ClearType::All))?;

    let line = l.readline_with_initial("Save: ", (&file, ""))?;

    fs::write(line, out)?;

    Ok(())
}
