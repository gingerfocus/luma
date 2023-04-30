#![feature(round_char_boundary)]

use crossterm::{
    cursor,
    event::{read, Event, KeyCode::*},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    cmp::{max, min},
    fs,
    io::{stdout, Error},
    sync::mpsc::{self, Sender},
};

fn main() -> Result<(), Error> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    let content = fs::read_to_string("/home/focus/code/lynkd/links.md").unwrap();

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
            Mode::Prompt => {
                let msg = format!("Delete \"{}\"? [y/n]", entries.get(index).unwrap().title);
                execute!(stdout, cursor::MoveTo(0, height as u16)).unwrap();
                print!("{}", msg);
                execute!(stdout, cursor::MoveTo(msg.len() as u16, height as u16)).unwrap()
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
                (Char('q'), Mode::Normal) => run = false,
                (Char('j') | Down, Mode::Normal) => index += 1,
                (Char('k') | Up, Mode::Normal) => index = index.saturating_sub(1),
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
                (Char('m'), Mode::Normal) => {
                    cur_mode =
                        Mode::Rename(EditableWord::new(entries.get(index).unwrap().title.clone()));
                }
                (Char('d'), Mode::Normal) => cur_mode = Mode::Prompt,
                (Char('y'), Mode::Prompt) => {
                    entries.remove(index);
                    cur_mode = Mode::Normal;
                }
                (Char('n'), Mode::Prompt) => cur_mode = Mode::Normal,
                (Left | Char('h'), Mode::Normal) => {
                    let entry = entries.remove(index);
                    entries.insert(index - 1, entry);
                    index -= 1;
                }
                (Right | Char('l'), Mode::Normal) => {
                    let entry = entries.remove(index);
                    entries.insert(index + 1, entry);
                    index += 1;
                }
                (Right, Mode::Rename(word)) => word.right(),
                (Left, Mode::Rename(word)) => word.left(),
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

    fs::write("/home/focus/code/lynkd/links.md", out)
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
    Prompt,
}

// TODO: have this take a mutable reference to a string
// it is garenteed to out live it as it is reference from
// the main file content slice
struct EditableWord {
    word: String,
    cursor: usize,
}

// Note this code crashes on non-utf8 characters
// as it trys to remove a byte from inside a character
impl EditableWord {
    fn new(word: String) -> Self {
        let cursor = word.len();
        Self { word, cursor }
    }

    fn add(&mut self, c: char) {
        if self.cursor < self.word.len() {
            self.word.insert(self.cursor, c);
        } else {
            self.word.push(c);
        }
        self.cursor += 1;
    }

    /// Removes the character that the cursor is over regardless of
    /// its size in bytes
    fn del(&mut self) {
        let slice = self.word.as_str();
        let bot = slice.floor_char_boundary(self.cursor.saturating_sub(1));
        let top = slice.ceil_char_boundary(self.cursor);

        let start = &slice[..bot];
        let end = {
            if top == slice.len() {
                ""
            } else {
                &slice[top..]
            }
        };
        self.word = format!("{}{}", start, end);

        self.cursor = bot;
    }

    fn left(&mut self) {
        self.cursor = self.word.floor_char_boundary(self.cursor.saturating_sub(1));
    }

    fn right(&mut self) {
        if self.cursor == self.word.len() {
            return;
        }
        self.cursor = self.word.ceil_char_boundary(self.cursor + 1);
    }
}
