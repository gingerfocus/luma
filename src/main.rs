#![feature(round_char_boundary)]

use crossterm::{
    cursor,
    event::{read, Event, KeyCode::*},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use editable_word::EditableWord;
use std::{
    cmp::{max, min},
    fs,
    io::{stdout, Error},
    process,
    sync::mpsc::{self, Sender},
};

fn main() -> Result<(), Error> {
    let arg = std::env::args().nth(1);
    let file = match arg {
        Some(f) => f,
        None => {
            eprintln!("Please provide a file to open");
            process::exit(1);
        }
    };

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    let content = fs::read_to_string(&file).unwrap();

    let mut entries = content
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            // TODO destructure using a regex
            let items: Vec<&str> = line.split("](").take(2).collect();
            let title = items[0].replacen('[', "", 1);
            let link = items[1].replacen(')', "", 1);
            Entry { link, title }
        })
        .collect::<Vec<Entry>>();

    let (tx, rx) = mpsc::channel();

    let rd = Reader { port: tx };

    std::thread::spawn(move || {
        rd.read_events().unwrap();
    });

    let mut index = 0;
    let mut cur_mode = Mode::Normal;
    let mut run = true;
    while run {
        execute!(stdout, Clear(ClearType::All)).unwrap();

        let (_, height) = term_size::dimensions().unwrap();
        let starting = max(index as i32 - 5, 0) as usize;

        entries
            .iter()
            .skip(starting)
            .take(height - 1)
            .enumerate()
            .for_each(|(j, entry)| {
                execute!(stdout, cursor::MoveTo(0, j as u16)).unwrap();
                print!("{}: {}", j + starting, entry.title);
            });

        match &mut cur_mode {
            Mode::Normal => {
                let cursor_pos = min(index as u16, 5);
                execute!(stdout, cursor::MoveTo(0, cursor_pos)).unwrap();
            }
            Mode::Rename(word) => {
                execute!(stdout, cursor::MoveTo(0, height as u16)).unwrap();
                let msg = format!("R: {}", word.word);
                print!("{}", msg);
                execute!(
                    stdout,
                    cursor::MoveTo(word.cursor as u16 + 3, height as u16)
                )
                .unwrap();
            }
            Mode::Delete => {
                let msg = format!("Delete \"{}\"? [y/n]", entries.get(index).unwrap().title);
                execute!(stdout, cursor::MoveTo(0, height as u16)).unwrap();
                print!("{}", msg);
                execute!(stdout, cursor::MoveTo(msg.len() as u16, height as u16)).unwrap()
            }
            Mode::Add(title, _, Editing::Title) => {
                let msg = format!("Title: {}", title.word);
                execute!(stdout, cursor::MoveTo(0, height as u16)).unwrap();
                print!("{}", msg);
                execute!(
                    stdout,
                    cursor::MoveTo(title.cursor as u16 + 7, height as u16)
                )?;
            }
            Mode::Add(_, link, Editing::Link) => {
                let msg = format!("Link: {}", link.word);
                execute!(stdout, cursor::MoveTo(0, height as u16)).unwrap();
                print!("{}", msg);
                execute!(
                    stdout,
                    cursor::MoveTo(link.cursor as u16 + 6, height as u16)
                )?;
            }
        }

        // sleep
        let event = rx.recv().unwrap();

        if let Event::Key(kd) = event {
            match (kd.code, &mut cur_mode) {
                (Esc, _) => {
                    cur_mode = Mode::Normal;
                }
                (Char(c), Mode::Rename(word)) => word.add(c),
                (Backspace, Mode::Rename(word)) => word.del(),
                (Char(c), Mode::Add(title, _, Editing::Title)) => title.add(c),
                (Backspace, Mode::Add(title, _, Editing::Title)) => title.del(),
                (Char(c), Mode::Add(_, link, Editing::Link)) => link.add(c),
                (Backspace, Mode::Add(_, link, Editing::Link)) => link.del(),
                (Char('q'), Mode::Normal) => run = false,
                (Char('j') | Down, Mode::Normal) => {
                    if index < entries.len() - 1 {
                        index += 1
                    }
                }
                (Char('k') | Up, Mode::Normal) => index = index.saturating_sub(1),
                (Char('g'), Mode::Normal) => index = 0,
                (Char('G'), Mode::Normal) => index = entries.len() - 1,
                (Char('i'), Mode::Normal) => {
                    cur_mode = Mode::Add(
                        EditableWord::new("".to_string()),
                        EditableWord::new("".to_string()),
                        Editing::Title,
                    )
                }
                (Enter, Mode::Rename(word)) => {
                    let entry = entries.get_mut(index).unwrap();
                    entry.title = word.word.clone();
                    cur_mode = Mode::Normal;
                }
                (Enter, Mode::Normal) => {
                    let entry = entries.get(index).unwrap();
                    let link = entry.link.clone();
                    _ = std::process::Command::new("xdg-open")
                        .arg(link)
                        .output()
                        .unwrap();
                }
                (Enter, Mode::Add(title, link, mode)) => match mode {
                    Editing::Title => mode.next(),
                    Editing::Link => {
                        entries.insert(
                            index,
                            Entry {
                                title: title.word.clone(),
                                link: link.word.clone(),
                            },
                        );
                        cur_mode = Mode::Normal;
                    }
                },
                (Tab, Mode::Add(_, _, mode)) => mode.next(),
                (Char('m'), Mode::Normal) => {
                    cur_mode =
                        Mode::Rename(EditableWord::new(entries.get(index).unwrap().title.clone()));
                }
                (Char('d'), Mode::Normal) => cur_mode = Mode::Delete,
                (Char('y'), Mode::Delete) => {
                    entries.remove(index);
                    cur_mode = Mode::Normal;
                }
                (Char('n'), Mode::Delete) => cur_mode = Mode::Normal,
                (Left | Char('h'), Mode::Normal) => {
                    if index > 0 {
                        let entry = entries.remove(index);
                        entries.insert(index - 1, entry);
                        index -= 1;
                    }
                }
                (Right | Char('l'), Mode::Normal) => {
                    if index < entries.len() - 1 {
                        let entry = entries.remove(index);
                        entries.insert(index + 1, entry);
                        index += 1;
                    }
                }
                // Various Cursor movment handles
                (Right, Mode::Rename(word)) => word.right(),
                (Left, Mode::Rename(word)) => word.left(),
                (Right, Mode::Add(_, link, Editing::Link)) => link.right(),
                (Left, Mode::Add(_, link, Editing::Link)) => link.left(),
                (Right, Mode::Add(title, _, Editing::Title)) => title.right(),
                (Left, Mode::Add(title, _, Editing::Title)) => title.left(),
                (_, _) => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    let out = entries
        .iter()
        .map(|entry| format!("[{}]({})", entry.title, entry.link))
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(&file, out)
}

struct Entry {
    link: String,
    title: String,
}

struct Reader {
    port: Sender<Event>,
}

impl Reader {
    fn read_events(&self) -> Result<(), Error> {
        loop {
            self.port.send(read()?).unwrap();
        }
    }
}

enum Mode {
    Normal,
    Rename(EditableWord),
    Delete,
    Add(EditableWord, EditableWord, Editing),
}

enum Editing {
    Link,
    Title,
}

impl Editing {
    fn next(&mut self) {
        match self {
            Editing::Link => *self = Editing::Title,
            Editing::Title => *self = Editing::Link,
        }
    }
}
