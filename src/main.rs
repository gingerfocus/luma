//! A Program to unite the web and filesystem

// #![warn(unused_crate_dependencies)]
// #![warn(missing_docs)]

mod app;
mod cli;
mod event;
mod input;
mod prelude;
mod state;
mod ui;

use crate::prelude::*;

use std::os::fd::FromRawFd;
use std::path::PathBuf;
use std::time::Duration;

use app::App;
use clap::Parser;

#[derive(Debug)]
enum LumaError {
    Input,
    Parse,
    Event,
    Render,
}

impl fmt::Display for LumaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LumaError::Input => f.write_str("input file not valid"),
            LumaError::Parse => f.write_str("could not parse input as luma"),
            LumaError::Event => f.write_str("could not read terminal event"),
            LumaError::Render => f.write_str("render pass failed"),
        }
    }
}
impl Context for LumaError {}

fn main() -> Result<(), LumaError> {
    let args = cli::Args::parse();

    args.log.then(|| init_logger(args.file));

    let f = fs::File::open(&args.input).change_context(LumaError::Input)?;
    let luma: Luma = json::from_reader(f).change_context(LumaError::Parse)?;
    // let luma = Luma {
    //     tabs: Vec::from([(
    //         String::from("audios"),
    //         Vec::from([
    //             Link {
    //                 name: String::from("example"),
    //                 link: String::from("https://example.com"),
    //                 ..Default::default()
    //             },
    //             Link {
    //                 name: String::from("anilist"),
    //                 link: String::from("https://anilist.co"),
    //                 ..Default::default()
    //             },
    //         ]),
    //     )]),
    // };

    // Safety: "I do solumnly swear that this is the only way I will write to
    // stdout and understand that if I choose to do it in any additional way I
    // will be remain content with my program exploding."
    //      - Evan Stokdyk, 3/18/2024
    let stdout = unsafe { fs::File::from_raw_fd(1) };

    let mut app = App::new(luma, stdout);

    app.redraw()
        .change_context(LumaError::Render)
        .attach_printable("Could perform first render. Is your terminal ok?")?;

    // --------------------------------------------
    while !app.stat.quit {
        log::debug!("starting event loop.");
        if let Some(e) = read_event()? {
            app.event(e);
        }
        if app.stat.draw {
            app.draw().change_context(LumaError::Render)?;
            app.stat.draw = false;
        }
    }
    // -----------------------------------------------

    let (luma, _term) = app.finish();

    let f = fs::File::create("out.json").unwrap();
    json::to_writer_pretty::<_, Luma>(f, &luma).unwrap();
    // json::to_writer::<_, Luma>(f, &luma).unwrap();

    log::trace!("exit.");

    Ok(())

    // HACK: at the end of the block the [`App`] is dropped. It contains a owned
    // handle to stdout and so will close it. Returning a result from this
    // function causes it to _print to stderr which is still open at that point
    // an so the error displays correctly.
}

fn read_event() -> Result<Option<crossterm::event::Event>, LumaError> {
    if crossterm::event::poll(Duration::from_millis(200)).change_context(LumaError::Event)? {
        let e = crossterm::event::read().change_context(LumaError::Event)?;
        Ok(Some(e))
    } else {
        Ok(None)
    }
}

fn init_logger(file: Option<PathBuf>) {
    let file = file.unwrap_or_else(|| {
        PathBuf::from(env::var("XDG_CACHE_HOME").unwrap_or_else(|_| {
            let home = env::var("HOME").expect("You don't have a $HOME???");
            format!("{home}/.cache")
        }))
        .join("luma.log")
    });

    let Ok(file) = fs::File::create(&file) else {
        eprintln!("could not open file to log: {:?}", file);
        return;
    };

    let _ = simplelog::WriteLogger::init(
        simplelog::LevelFilter::Trace,
        simplelog::Config::default(),
        file,
    );

    log::debug!("log init");
}
