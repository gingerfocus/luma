//! A Program to unite the web and filesystem

#![warn(unused_crate_dependencies)]
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

fn eloop<B: tui::backend::Backend + io::Write>(app: &mut App<B>) -> Result<(), LumaError> {
    app.redraw()
        .change_context(LumaError::Render)
        .attach_printable("Could perform first render. Is your terminal ok?")?;

    // --------------------------------------------
    while !app.state.quit {
        log::debug!("starting event loop.");
        if let Some(e) = read_event()? {
            app.event(e).change_context(LumaError::Event)?;
        }
        if app.state.draw {
            app.draw().change_context(LumaError::Render)?;
            app.state.draw = false;
        }
    }
    // -----------------------------------------------

    Ok(())
}

fn main() -> Result<(), LumaError> {
    let args = cli::parse();

    std::panic::set_hook(Box::new(|info| {
        // it is ok to violate the safety I will talk about in 12 lines of code
        // beacuse this is the only code that will run during a panic.
        let mut stdout = std::io::stdout();

        app::deinit(&mut stdout);

        log::error!("{}", info);
        eprintln!("{}", info);
    }));

    args.log.then(|| init_logger(args.file));

    let f = fs::File::open(&args.input).change_context(LumaError::Input)?;
    let luma: Luma = json::from_reader(f).change_context(LumaError::Parse)?;

    // Safety: "I do solumnly swear that this is the only way I will write to
    // stdout and understand that if I choose to do it in any additional way I
    // will remain happy when my program explodes."
    //      - Evan Stokdyk, 3/18/2024
    let mut stdout = unsafe { fs::File::from_raw_fd(1) };

    app::init(&mut stdout);
    let mut app = App::new(luma, stdout);

    let res = eloop(&mut app);

    let (luma, mut term) = app.finish();
    app::deinit(term.backend_mut());
    res?;

    // .change_context(LumaError::Panic);

    let f = fs::File::create("out.json").change_context(LumaError::Input)?;
    json::to_writer_pretty::<_, Luma>(f, &luma).change_context(LumaError::Parse)?;

    log::trace!("exit.");

    Ok(())

    // HACK: at the end of the block the [`App`] is dropped. It contains a owned
    // handle to stdout and so will close it. Returning a result from this
    // function causes it to _print to stderr which is still open at that point
    // an so the error displays correctly.
}

fn read_event() -> Result<Option<crossterm::event::Event>, LumaError> {
    if crossterm::event::poll(Duration::from_secs(3)).change_context(LumaError::Event)? {
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

use pty_process as _;
// fn a() {
//     use std::io::Read;
//     let mut pty = pty_process::blocking::Pty::new().unwrap();
//
//     let (x, y) = crossterm::terminal::size().unwrap();
//     pty.resize(pty_process::Size::new(x, y)).unwrap();
//
//     let mut cmd = pty_process::blocking::Command::new("nvim");
//     let mut child = cmd.spawn(&pty.pts().unwrap()).unwrap();
//     loop {
//         if child.try_wait().unwrap().is_some() {
//             break;
//         }
//         let mut buf = [0u8; 256];
//         let len = pty.read(&mut buf).unwrap();
//         if len == 0 {
//             break;
//         }
//         use std::io::Write;
//         std::io::stdout().write_all(&buf[..len]).unwrap();
//         // let s = std::str::from_utf8(&buf[..len]).unwrap();
//         // log::info!("{}", s);
//     }
// }
