//! A Program to unite the web and filesystem

#![feature(lazy_cell)]
// #![allow(unused)]
#![warn(unused_crate_dependencies)]
// #![warn(missing_docs)]

pub mod app;
// pub mod aud;
pub mod cli;
pub mod error;
mod input;
pub mod mode;
pub mod prelude;
pub mod state;
#[macro_use]
pub mod util;
pub mod ui;

use crate::prelude::*;
use clap::Parser;
use tokio::sync::oneshot;

pub enum LumaMessage {
    Redraw,
    Exit,
    SetMode(Mode),
    AskQuestion(QuestionComponents, oneshot::Sender<String>),
}

pub struct QuestionComponents {
    name: String,
    _question: QuestionType,
    default: String,
}

pub enum QuestionType {
    Editor,
}

/// The main function is the cordinator for our tokio rt. It spawns a number of
/// task that all coridnate to make the app run:
/// - The render thread
/// - The input thread
/// - The worker/processor thread
#[tokio::main]
async fn main() -> Result<()> {
    let args = crate::cli::Args::parse();

    *LUMA.write().unwrap() = json::from_str(&fs::read_to_string(&args.file)?)?;

    env_logger::builder()
        .target(env_logger::Target::Pipe(Box::new(fs::File::create(
            "./luma.log",
        )?)))
        .init();

    log::debug!("Luma log init");

    let mode: GlobalMode = Default::default();

    let (tx_reqs, rx_reqs) = std::sync::mpsc::channel();
    let (tx_input, rx_input) = std::sync::mpsc::channel();

    let input_task = tokio::spawn(async move {
        let mut screen = Screen::default();
        screen.init().unwrap();
        loop {
            let event = read_event().await;
            log::debug!("read key event: {:?}", event);
            if tx_input.send(event).is_err() {
                break;
            }
        }
        screen.deinit().unwrap();
    });

    let mode_c = mode.clone();
    let request_task = tokio::spawn(async move {
        let mode = mode_c;
        let mut handler = Handler::new();

        while let Ok(e) = rx_input.recv() {
            log::debug!("got key event: {:?}", e);

            let Some(req) = ({
                let mode: &mut Mode = &mut mode.write().unwrap();
                input::handle(e, &mut handler, mode)
            }) else {
                continue;
            };

            if tx_reqs.send(req).is_err() {
                break;
            }
        }
    });

    let render_task = tokio::spawn(async move {
        let mut app = App::new().unwrap();
        app.init().unwrap();

        // --------------------------------------------
        app.draw(&mode.read().unwrap()).unwrap();

        while app.run {
            match rx_reqs.recv().ok() {
                Some(LumaMessage::Redraw) => {
                    app.draw(&mode.read().unwrap()).unwrap();
                }
                Some(LumaMessage::Exit) => app.run = false,
                Some(LumaMessage::SetMode(m)) => {
                    *mode.write().unwrap() = m;
                    app.draw(&mode.read().unwrap()).unwrap();
                }
                Some(LumaMessage::AskQuestion(comps, resp)) => {
                    app.deinit().unwrap();
                    let q = requestty::Question::editor(comps.name)
                        .default(comps.default)
                        .build();
                    let ans = requestty::prompt_one(q)
                        .unwrap()
                        .as_string()
                        .unwrap()
                        .to_owned();
                    resp.send(ans).unwrap();
                    app.init().unwrap();
                }
                None => {}
            }

            // LumaMessage::AskQuestion(q, callback) => {
            //     app.deinit()?;
            //     let ans = requestty::prompt_one(q).unwrap();
            //     callback(ans);
            //     app.init()?;
            //     app.redraw()?;
            // }
        }

        app.deinit().unwrap();
        // --------------------------------------------

        crossterm::execute!(io::stdout(), crossterm::cursor::Show).unwrap();
    });

    render_task.await.unwrap();
    request_task.await.unwrap();
    input_task.await.unwrap();

    let file = requestty::Question::input("file")
        .message("Save as")
        .default(args.file.to_str().unwrap())
        .build();
    let path = match requestty::prompt_one(file) {
        Ok(requestty::Answer::String(s)) => s,
        _ => unreachable!(),
    };

    if let Ok(f) = fs::File::create(path) {
        json::to_writer_pretty::<_, Luma>(f, &LUMA.read().unwrap()).unwrap();
    }

    Ok(())
}

async fn read_event() -> Event {
    crossterm::event::read()
        .map(Into::into)
        .unwrap_or(Event::Tick)
    // if crossterm::event::poll(std::time::Duration::from_millis(500)).unwrap() {
    //     crossterm::event::read().map(Into::into).unwrap()
    // } else {
    //     Event::Tick
    // }
}
