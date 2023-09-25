pub mod app;
pub mod cli;
pub mod event;
pub mod luma;
pub mod mode;
pub mod prelude;
pub mod state;
pub mod ui;
pub mod util;

use crate::prelude::*;
use clap::Parser;

pub enum LumaMessage {
    Redraw,
    SetMode(Mode),
    Exit,
    Nothing,
}

fn main() -> Result<()> {
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

    crossterm::execute!(io::stdout(), crossterm::cursor::Show)?;
    let file = requestty::Question::input("file")
        .message("Save as")
        .default(args.file.to_str().unwrap())
        .build();
    let path = match requestty::prompt_one(file) {
        Ok(requestty::Answer::String(s)) => s,
        _ => unreachable!(),
    };

    if let Ok(f) = fs::File::create(path) {
        yaml::to_writer(f, &luma).unwrap();
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
