#![feature(result_option_inspect)]

// mod render;

use anyhow::Result;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode::*},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use rustyline::{history::FileHistory, Editor};
use std::{
    cmp::{max, min},
    fs,
    io::{
        stdout,
        BufRead,
        BufReader,
    },
    process,
};

struct Entry {
    link: String,
    title: String,
    notes: Vec<String>,
    folded: bool,
}

fn main() -> Result<()> {
    // use simplelog::WriteLogger;
    // WriteLogger::init(
    //     simplelog::LevelFilter::Info,
    //     simplelog::Config::default(),
    //     fs::File::create("lynkd.log")?,
    // )?;

    let file = std::env::args().nth(1).unwrap_or_else(|| {
        eprint!("Please provide a file to open");
        process::exit(1);
    });

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    // let content = fs::read_to_string(&file).unwrap();


    let mut entries: Vec<Entry> = Vec::new();
    entries.push(Entry {
        link: "root".to_string(),
        title: "test".to_string(),
        notes: Vec::new(),
        folded: false,
    });


    let f = fs::File::open(&file)?;
    let rd = BufReader::new(f);
    
    let mut is_first = true;

    for l in rd.lines() {
        let l = l?;
        if l.is_empty() {
            continue;
        }
        if l.starts_with('[') && l.ends_with(')') {
            if is_first {
                entries.pop();
            }
            let items: Vec<&str> = l.split("](").take(2).collect();
            let title = items[0].replacen('[', "", 1);
            let link = items[1].replacen(')', "", 1);
            entries.push(Entry {
                link,
                title,
                notes: Vec::new(),
                folded: false,
            });
        } else {
            let last = entries.last_mut().unwrap();
            last.notes.push(l);
        }
        is_first = false;
    }

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

    let mut index = 0;
    let mut run = true;

    while run {
        execute!(stdout, Clear(ClearType::All)).unwrap();

        let (_, height) = term_size::dimensions().unwrap();

        let starting = max(index as i32 - 5, 0) as usize;

        let mut print = 0;

        entries
            .iter()
            .skip(starting)
            .take(height - 1)
            .enumerate()
            .for_each(|(j, entry)| {
                execute!(stdout, cursor::MoveTo(0, print as u16)).unwrap();
                print!("{}: {}", j + starting, entry.title);
                print += 1;
                if !entry.folded {
                    for l in entry.notes.iter() {
                        execute!(stdout, cursor::MoveTo(0, print as u16)).unwrap();
                        print!("\t{}", l);
                        print += 1;
                    }
                }
                
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
                    if index < entries.len() - 1 {
                        index += 1
                    }
                }
                Char('k') | Up => index = index.saturating_sub(1),
                Char('g') => index = 0,
                Char('G') => index = entries.len() - 1,
                Char('i') => {
                    execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    let title = l.readline("Title: ")?;

                    execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    let url = l.readline("URL: ")?;

                    entries.insert(index, Entry {
                        title,
                        link: url,
                        notes: Vec::new(), 
                        folded: true
                    });
                }
                Enter => {
                    let entry = entries.get(index).unwrap();
                    let link = entry.link.clone();
                    _ = std::process::Command::new("xdg-open")
                        .arg(link)
                        .output()
                        .unwrap();
                }
                Char('m') => {
                    execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    let old = entries.get_mut(index).unwrap();

                    l.readline_with_initial("Rename: ", (old.title.as_str(), ""))
                        .inspect(|name| {
                            old.title = name.clone();
                        })?;
                }
                Char('d') => {
                    execute!(stdout, cursor::MoveTo(0, height as u16))?;
                    let r = l.readline("Delete [y/N]: ");
                    match r {
                        Ok(c) if c == "y" => {
                            entries.remove(index);
                        }
                        _ => {}
                    };
                }
                Left | Char('h') => {
                    if index > 0 {
                        let entry = entries.remove(index);
                        entries.insert(index - 1, entry);
                        index -= 1;
                    }
                }
                Right | Char('l') => {
                    if index < entries.len() - 1 {
                        let entry = entries.remove(index);
                        entries.insert(index + 1, entry);
                        index += 1;
                    }
                }
                _ => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    let out = entries
        .iter()
        .map(|entry| {
            let link = format!("[{}]({})\n", entry.title, entry.link);
            link + entry.notes
                .iter()
                .map(|l| format!("\t{}", l))
                .collect::<Vec<String>>()
                .join("\n").as_str()
        })
        .collect::<Vec<String>>()
        .join("\n");

    execute!(stdout, Clear(ClearType::All))?;

    let line = l.readline_with_initial("Save: ", (&file, ""))?;

    fs::write(line, out)?;

    Ok(())
}
