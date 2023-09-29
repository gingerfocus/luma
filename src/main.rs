//! A Program to unite the web and filesystem

// #![allow(unused)]
#![warn(unused_crate_dependencies)]
// #![warn(missing_docs)]

pub mod app;
// pub mod aud;
pub mod cli;
pub mod error;
pub mod input;
pub mod mode;
pub mod prelude;
pub mod state;
pub mod ui;
pub mod util;

use std::{env, path::PathBuf, time::Duration};

use crate::prelude::*;
use clap::Parser;
use tokio::{sync::mpsc, task::JoinHandle};

pub enum LumaMessage {
    Redraw,
    Exit,
    SetMode(Mode),
    AddHandle(JoinHandle<Vec<LumaMessage>>),
    // AskQuestion(QuestionComponents, oneshot::Sender<String>),
}

// pub struct QuestionComponents {
//     name: String,
//     _question: QuestionType,
//     default: String,
// }
// pub enum QuestionType {
//     Editor,
// }

async fn init_logger() -> Result<()> {
    let home = env::var("HOME").expect("You don't have a $HOME???");
    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        fs::File::create(format!("{home}/.cache/luma.log"))?,
    )
    .unwrap();

    log::debug!("Luma log init");
    Ok(())
}

/// The main function is the cordinator for our tokio rt. It spawns a number of
/// task that all coridnate to make the app run:
/// - The render thread
/// - The input thread
/// - The worker/processor thread
#[tokio::main]
async fn main() -> Result<()> {
    let args = crate::cli::Args::parse();

    init_logger().await?;

    *LUMA.write().await = json::from_str(&fs::read_to_string(&args.file)?)?;
    let mode: GlobalMode = Default::default();

    let (tx_input, rx_input) = mpsc::channel(16);
    let (tx_reqs, rx_reqs) = mpsc::channel(32);

    let input_task = tokio::spawn(read_events(tx_input));
    let request_task = tokio::spawn(process_request(tx_reqs, rx_input, mode.clone()));
    let render_task = tokio::spawn(render_screen(rx_reqs, mode));

    render_task.await.unwrap();
    input_task.await.unwrap();
    request_task.await.unwrap();

    crossterm::execute!(io::stdout(), crossterm::cursor::Show).unwrap();

    save_luma(args.file).await;

    Ok(())
}

async fn save_luma(file: PathBuf) {
    let file = requestty::Question::input("file")
        .message("Save as")
        .default(file.to_str().unwrap())
        .build();
    let path = match requestty::prompt_one(file) {
        Ok(requestty::Answer::String(s)) => s,
        _ => unreachable!(),
    };

    if let Ok(f) = fs::File::create(path) {
        json::to_writer_pretty::<_, Luma>(f, &LUMA.read().await as &Luma).unwrap();
    }
}

async fn read_events(tx: mpsc::Sender<Event>) {
    log::info!("event thread created");
    let mut reader = crossterm::event::EventStream::new();

    let mut screen = Screen::default();
    screen.init().unwrap();

    'read: loop {
        let delay = tokio::time::sleep(Duration::from_secs(1));
        let event = futures::StreamExt::next(&mut reader);

        let event = tokio::select! {
            _ = tx.closed() => break 'read,
            _ = delay => Event::Tick,
            e = event => {
                if let Some(event) = e {
                    let event = event.unwrap().into();
                    log::debug!("read key event: {:?}", event);
                    event
                } else {
                    break;
                }
            }
        };

        if tx.send(event).await.is_err() {
            break 'read;
        }
    }
    screen.deinit().unwrap();

    log::info!("event thread ended");
}

async fn process_request(
    tx: mpsc::Sender<LumaMessage>,
    mut rx: mpsc::Receiver<Event>,
    mode: GlobalMode,
) {
    log::info!("request thread created");
    let mut handler = Handler::new();

    'read: loop {
        let timeout = tokio::time::sleep(Duration::from_secs(2));

        tokio::select! {
            _ = tx.closed() => break 'read,
            _ = timeout => return,
            e = rx.recv() => {
                if let Some(event) = e {
                    log::trace!("got key event: {:?}", event);

                    for req in input::handle(event, &mut handler, &mode).await {
                        if tx.send(req).await.is_err() {
                            break;
                        }
                    }
                }
            }
        }
    }
    log::info!("request thread ended");
}

async fn render_screen(mut rx: mpsc::Receiver<LumaMessage>, mode: GlobalMode) {
    log::info!("render thread created");

    let mut handles: Vec<JoinHandle<Vec<LumaMessage>>> = vec![];
    let mut app = App::new().unwrap();
    app.init().unwrap();

    // --------------------------------------------
    app.draw(&mode.read().unwrap()).unwrap();

    async fn handle_msg(
        msg: LumaMessage,
        app: &mut App,
        mode: &GlobalMode,
        handles: &mut Vec<JoinHandle<Vec<LumaMessage>>>,
    ) {
        match msg {
            LumaMessage::Redraw => {
                app.draw(&mode.read().unwrap()).unwrap();
            }
            LumaMessage::Exit => {
                app.run = false;
            }
            LumaMessage::SetMode(m) => {
                log::info!("waiting on write for mode.");
                *mode.write().unwrap() = m;
                app.draw(&mode.read().unwrap()).unwrap();
            }
            // LumaMessage::AskQuestion(comps, resp) => {
            //     app.deinit().unwrap();
            //     let q = requestty::Question::editor(comps.name)
            //         .default(comps.default)
            //         .build();
            //     let ans = requestty::prompt_one(q)
            //         .unwrap()
            //         .as_string()
            //         .unwrap()
            //         .to_owned();
            //     resp.send(ans).unwrap();
            //     app.init().unwrap();
            // }
            LumaMessage::AddHandle(h) => handles.push(h),
        }
    }

    while app.run {
        if let Some(msg) = rx.recv().await {
            handle_msg(msg, &mut app, &mode, &mut handles).await;
        }

        let mut new_handles = vec![];
        for h in handles {
            if h.is_finished() {
                if let Ok(msgs) = h.await {
                    for msg in msgs {
                        handle_msg(msg, &mut app, &mode, &mut new_handles).await;
                    }
                }
            } else {
                new_handles.push(h);
            }
        }
        handles = new_handles;

        // if let Ok(Some(msg)) = block_on(h) {
        //     handle_msg(msg, &mut app, &mode, &mut handles);
        // }
        // None
    }

    app.deinit().unwrap();
    // --------------------------------------------

    log::info!("render thread ended");
}
