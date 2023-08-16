mod app;
mod cli;
mod event;
mod luma;
mod mode;
mod prelude;
mod state;
mod ui;
mod util;

// mod vault;

pub extern crate serde_yaml as yaml;

use clap::Parser;

use crate::ui::screen::Screen;

use crate::prelude::*;

pub enum LumaMessage {
    Redraw,
    SetMode(Mode),
    Exit,
    Nothing,
}

fn main() -> anyhow::Result<()> {
    let args = crate::cli::Args::parse();

    // let log_file = std::fs::File::create("./mumi.log").unwrap();
    // env_logger::builder()
    //     .target(env_logger::Target::Pipe(Box::new(log_file)))
    //     .init();

    // let mut state = State::default();
    let mut mode = Mode::default();
    let mut luma: Luma = yaml::from_str(&fs::read_to_string(&args.file)?)?;
    // let mut client = mpd::Client::new();

    let mut screen = Screen::default();
    // HACK: make the screen valid during the transition period
    screen.init()?;

    let mut app = App::new()?;

    // --------------------------------------------
    app.init()?;
    app.draw(&luma, &mode)?;

    while app.run {
        let event = read_event();
        match app.handle(event, &mut luma, &mut mode)? {
            LumaMessage::Redraw => app.draw(&luma, &mode)?,
            LumaMessage::SetMode(m) => {
                mode = m;
                app.draw(&luma, &mode)?;
            }
            LumaMessage::Exit => app.run = false,
            LumaMessage::Nothing => {}
        }
    }

    app.deinit()?;
    // --------------------------------------------

    println!("Save file? (y/N)");
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;

    if response.trim().starts_with('y') {
        println!("Save as? (Default: {})", args.file.to_str().unwrap());
        let mut save_file = String::new();
        io::stdin().read_line(&mut save_file)?;
        // remove new line from reading stding
        let mut save_file = std::path::PathBuf::from(save_file.trim().to_string());
        if !save_file.exists() {
            save_file = args.file;
        }
        fs::write(save_file, yaml::to_string(&luma)?.as_bytes())?;
    }

    Ok(())
}

fn read_event() -> Event {
    if crossterm::event::poll(std::time::Duration::from_millis(500)).unwrap() {
        crossterm::event::read().map(Into::into).unwrap()
    } else {
        Event::Tick
    }
}
