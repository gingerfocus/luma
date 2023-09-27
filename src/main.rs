//! A Program to unite the web and filesystem

#![feature(lazy_cell)]
// #![allow(unused)]
#![warn(unused_crate_dependencies)]
// #![warn(missing_docs)]

pub mod app;
pub mod aud;
pub mod cli;
mod input;
pub mod luma;
pub mod mode;
pub mod prelude;
pub mod ui;

use crate::prelude::*;
use clap::Parser;
use input::Handler;

lazy_static::lazy_static!(
    static ref AUDIO_OPENER: luma::OpenCommand = luma::OpenCommand {
        cmd: "firefox",
        args: ["--private-window"].into(),
    };
);
// const LINK_OPENER: LazyCell<luma::OpenCommand> = LazyCell::new(|| luma::OpenCommand {
//     cmd: "firefox",
//     args: ["--private-window"].into(),
// });
// const TEXT_OPENER: LazyCell<luma::OpenCommand> = LazyCell::new(|| luma::OpenCommand {
//     cmd: "firefox",
//     args: ["--private-window"].into(),
// });
// const DOWNLOAD_DIR: &'static str = "~/dl";

pub enum LumaMessage {
    Redraw,
    SetMode(Box<Mode>),
    Exit,
}

fn main() -> Result<()> {
    let args = crate::cli::Args::parse();

    // IT WORKS, but slow even on release
    // aud::tags::add_tag("/home/focus/code/luma/test-in.ogg", "hello", "world")?;

    let log_file = std::fs::File::create("./luma.log").unwrap();
    env_logger::builder()
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();
    log::debug!("log init");

    // let mut state = State::default();
    let mut mode = Box::<Mode>::default();
    let mut luma: Luma = json::from_str(&fs::read_to_string(&args.file)?)?;
    // let mut client = mpd::Client::new();

    let mut screen = Screen::default();
    // HACK: make the screen valid during the transition period
    screen.init()?;

    let mut app = App::new()?;
    let mut handler = Handler::new();
    handler.add_default_binds();

    // --------------------------------------------
    app.init()?;
    app.draw(&luma, &mode)?;

    while app.run {
        let event = read_event();
        log::trace!("read key event: {:?}", event);
        if let Some(req) = input::handle(event, &mut screen, &mut luma, &mut mode, &handler) {
            match req {
                LumaMessage::Redraw => {
                    log::trace!("redrawing");
                    app.draw(&luma, &mode)?;
                }
                LumaMessage::SetMode(m) => {
                    log::trace!("setting mode");
                    mode = m;
                    app.draw(&luma, &mode)?;
                }
                LumaMessage::Exit => app.run = false,
            }
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
        json::to_writer_pretty(f, &luma).unwrap();
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
