use std::{fs, io::{Error, Read, stdout}, sync::mpsc::Sender};
use crossterm::{event::{read, Event, KeyCode}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}, cursor};

fn main () { 
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    let content = fs::read_to_string("/home/focus/code/lynkd/links.md").unwrap();

    let mut entries = Vec::new();
    content.lines().filter(|l| !l.is_empty()).for_each(|line| {
        let items: Vec<&str> = line.split("](").take(2).collect();
        let title = items[0].replacen("[", "", 1);
        let link = items[1].replacen(")", "", 1);
        entries.push(Entry { link, title });
    });
    
    let (tx, rx) = std::sync::mpsc::channel();

    let rd = Reader { port: tx }; 

    std::thread::spawn(move || {
        rd.read_events().unwrap();
    });

    let mut index = 0;


    let mut run = true;
    while run {
        // get terminal size
        let (_, height) = term_size::dimensions().unwrap();
        entries.iter().skip(index).take(height as usize).enumerate().for_each(|(j, entry)| {
            execute!(stdout, cursor::MoveTo(0, j as u16)).unwrap();
            println!("{}: {}", j+index, entry.title);
        });
        
        execute!(stdout, cursor::MoveTo(0, 5)).unwrap();
        
        // sleep
        let event = rx.recv().unwrap();
        
        match event {
            Event::Key(k) => {
                match k.code {
                    KeyCode::Char('q') => run = false, 
                    KeyCode::Char('j') => index += 1,
                    KeyCode::Char('k') => {
                        if index > 0 {
                            index -= 1
                        }
                    }
                    _ => {}
                } 
            },
            _ => {}
        }
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    execute!(stdout, LeaveAlternateScreen).unwrap();
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

