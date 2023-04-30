use std::{
    fs,
    io::{Error, stdout},
    sync::mpsc::{Sender, self}, cmp::{max, min}
};
use crossterm::{
    event::{read, Event, KeyCode::*},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, ClearType, Clear},
    cursor
};

fn main () { 
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    let content = fs::read_to_string("/home/focus/code/lynkd/links.md").unwrap();

    let mut entries = content.lines().filter(|l| !l.is_empty()).map(|line| {
        let items: Vec<&str> = line.split("](").take(2).collect();
        let title = items[0].replacen("[", "", 1);
        let link = items[1].replacen(")", "", 1);
        Entry { link, title }
    }).collect::<Vec<Entry>>();
    
    let (tx, rx) = mpsc::channel();

    let rd = Reader { port: tx }; 

    std::thread::spawn(move || {
        rd.read_events().unwrap();
    });

    let mut index = 0;
    let mut cur_mode = Mode::Normal;
    let mut current_rename = String::new();
    let mut run = true;
    while run {
        // get terminal size
        //
        execute!(stdout, Clear(ClearType::All)).unwrap();

        let (_, height) = term_size::dimensions().unwrap();
        let starting = max(index as i32 - 5, 0) as usize;

        entries.iter().skip(starting).take((height-1) as usize).enumerate().for_each(|(j, entry)| {
            execute!(stdout, cursor::MoveTo(0, j as u16)).unwrap();
            print!("{}: {}", j+starting, entry.title);
        });

        match cur_mode {
            Mode::Normal => {
                let cursor_pos = min(index as u16, 5);
                execute!(stdout, cursor::MoveTo(0, cursor_pos)).unwrap();
            }
            Mode::Rename => {
                execute!(stdout, cursor::MoveTo(0, height as u16)).unwrap();
                let msg = format!("R: {}", current_rename);
                print!("{}", msg);
                execute!(stdout, cursor::MoveTo(msg.len() as u16, height as u16)).unwrap();
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
            match (kd.code, &cur_mode) {
                (Esc, _) => {
                    cur_mode = Mode::Normal;
                    current_rename = String::new();
                }
                (Char(c), Mode::Rename) => { current_rename.push(c) }
                (Backspace, Mode::Rename) => { current_rename.pop(); }
                (Char('q'), Mode::Normal) => run = false, 
                (Char('j'), Mode::Normal) => index += 1,
                (Char('k'), Mode::Normal) => if index > 0 { index -= 1 },
                (Enter, Mode::Rename) => {
                    let entry = entries.get_mut(index).unwrap();
                    entry.title = current_rename.clone();
                    cur_mode = Mode::Normal;
                    current_rename = String::new();
                }
                (Enter, Mode::Normal) => {
                    let entry = entries.get(index).unwrap();
                    let link = entry.link.clone();
                    let _ = std::process::Command::new("xdg-open").arg(link).spawn().unwrap();
                }
                (Char('m'), Mode::Normal) => {
                    cur_mode = Mode::Rename;
                    current_rename = entries.get(index).unwrap().title.clone();
                }
                (Char('d'), Mode::Normal) => cur_mode = Mode::Prompt, 
                (Char('y'), Mode::Prompt) => {
                    entries.remove(index);
                    cur_mode = Mode::Normal;
                }
                (Char('n'), Mode::Prompt) => cur_mode = Mode::Normal,
                (_, _) => {}
            }
        }
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    execute!(stdout, LeaveAlternateScreen).unwrap();

    let out = entries.iter().map(|entry| {
        format!("[{}]({})", entry.title, entry.link)
    }).collect::<Vec<String>>().join("\n");

    fs::write("/home/focus/code/lynkd/links.md", out).unwrap();    

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
    Rename, // { name: String },
    Prompt,
}
