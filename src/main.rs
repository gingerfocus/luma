// mod render;
// mod state;
mod link;

use anyhow::Result;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode::*},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use markdown::mdast::Node;
use rustyline::{history::FileHistory, Editor};
// use state::Link;

use std::{
    cmp::{max, min},
    fs,
    io::{stdout, BufReader},
    process,
};

fn main() -> Result<()> {
    let file = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please provide a file to open");
        process::exit(1);
    });

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    let content = fs::read_to_string(&file)?;
    let mut markdown = markdown::to_mdast(&content, &markdown::ParseOptions::default()).unwrap();

    // create the line reader that is used to get input from the user
    let mut l: Editor<(), FileHistory> = rustyline::Editor::with_config(
        rustyline::config::Builder::new()
            // .completion_type(rustyline::config::CompletionType::List)
            .edit_mode(rustyline::config::EditMode::Vi)
            .build(),
    )?;

    let mut index: usize = 0; // the place of the cursor along the screen

    let f = fs::File::open(&file)?;
    let rd = BufReader::new(f);

    let mut is_first = true;

    // .fold(Vec::new(), |mut acc, line| {
    //     if line.starts_with('[') && line.ends_with(')') {
    //         acc.push(line.to_string());
    //     } else {
    //         let last = acc.last_mut().unwrap();
    //         last.push_str(line);
    //     }
    //     acc
    // })
    // .filter(|l| l.starts_with('[') && l.ends_with(')'))
    // .map(|line| {
    //     // TODO destructure using a regex
    //     let items: Vec<&str> = line.split("](").take(2).collect();
    //     let title = items[0].replacen('[', "", 1);
    //     let link = items[1].replacen(')', "", 1);
    //     Entry { link, title }
    // })

    let builder = rustyline::config::Builder::new();
    let conf = builder
        // .completion_type(rustyline::config::CompletionType::List)
        .edit_mode(rustyline::config::EditMode::Vi)
        .build();

    let mut l: Editor<(), FileHistory> = rustyline::Editor::with_config(conf)?;

    let mut index: u32 = 0;
    let mut run = true;

    // let mut full_lines: Vec<Line> = markdown.lines();

    while run {
        execute!(stdout, Clear(ClearType::All)).unwrap();

        let (_, height) = term_size::dimensions().unwrap();

        let starting = max(index as i32 - 5, 0) as usize;

        let mut print = 0;

        // let mut active_lines: Vec<&mut Line> = Vec::new();

        markdown
            .children()
            .unwrap()
            .iter()
            .flat_map(|child| {
                if let Node::Link(l) = child {
                    Some(l)
                } else {
                    None
                }
            })
            .skip(starting)
            .take(height - 1)
            .enumerate()
            .for_each(|(i, child)| {
                execute!(stdout, cursor::MoveTo(0, print as u16)).unwrap();
                let t = &child.title;
                let text = t.clone().unwrap();
                print!("{}: {}", i + starting, text);
            });
        // full_lines
        //     .iter_mut()
        //     .skip(starting)
        //     .take(height - 1)
        //     .for_each(|line| {
        //         print!("{}", line.display());
        //
        //         print += 1;
        //
        //         // if !entry.folded {
        //         //     for l in entry.notes.iter() {
        //         //         execute!(stdout, cursor::MoveTo(0, print as u16)).unwrap();
        //         //         print!("\t{}", l);
        //         //         active_lines.push(Line::Note);
        //         //         print += 1;
        //         //     }
        //         // }
        //
        //         active_lines.push(line);
        //     });

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
                    // if index < markdown.sections.len() - 1 {
                    //     index += 1
                    // }
                }
                Char('k') | Up => index = index.saturating_sub(1),
                Char('g') => index = 0,
                // Char('G') => index = markdown.sections.len() - 1,
                Char('i') => {
                    // execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    let title = l.readline("Title: ")?;

                    // execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    let url = l.readline("URL: ")?;

                    // markdown
                    //     .current_section(index)
                    //     .links
                    //     .push(Link::new(title, url));
                }
                Enter => {
                    // if let Some(Line::Link(lnk, _sec)) = active_lines.get(index) {
                    //     lnk.open()?;
                    // }
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

    // let out = markdown
    //     .sections
    //     .iter()
    //     .map(|sec| sec.format())
    //     .collect::<Vec<String>>()
    //     .join("\n");

    // execute!(stdout, Clear(ClearType::All))?;

    let line = l.readline_with_initial("Save: ", (&file, ""))?;

    // fs::write(line, out)?;

    Ok(())
}
