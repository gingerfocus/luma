//! A Program to unite the web and filesystem

// #![warn(unused_crate_dependencies)]
// #![warn(missing_docs)]

mod app;
mod cli;
// pub mod error;
mod event;
// pub mod input;
mod prelude;
mod state;
// pub mod ui;
// pub mod util;

use crate::prelude::*;

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

    let mut app = App::new(luma);

    app.redraw()
        .change_context(LumaError::Render)
        .attach_printable("Could perform first render. Is your terminal ok?")?;

    // --------------------------------------------
    while !app.quit {
        log::debug!("starting event loop.");
        if let Some(e) = read_event()? {
            app.event(e);
        }
        if app.draw {
            app.redraw().change_context(LumaError::Render)?;
            app.draw = false;
        }
    }
    // -----------------------------------------------

    // let f = fs::File::create(path)?;
    // json::to_writer_pretty::<_, Luma>(f, &luma)?;

    log::trace!("exit.");
    Ok(())
}

fn read_event() -> Result<Option<crossterm::event::Event>, LumaError> {
    if crossterm::event::poll(Duration::from_secs(0)).change_context(LumaError::Event)? {
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
