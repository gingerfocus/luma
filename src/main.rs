//! A Program to unite the web and filesystem

// #![allow(unused)]
#![warn(unused_crate_dependencies)]
// #![warn(missing_docs)]

pub mod app;
// pub mod aud;
pub mod cli;
pub mod error;
pub mod event;
pub mod input;
pub mod mode;
pub mod prelude;
pub mod state;
pub mod ui;
#[macro_use]
pub mod util;

use std::{env, path::PathBuf};

use crate::prelude::*;
use clap::Parser;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

pub enum LumaMessage {
    Redraw,
    Exit,
    SetMode(Mode),
    AddHandle(JoinHandle<Vec<LumaMessage>>),
    OpenEditor {
        text: String,
        resp: oneshot::Sender<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = crate::cli::Args::parse();

    if args.log {
        init_logger(args.log_file).await?;
    }

    *LUMA.write().await = json::from_str(&fs::read_to_string(&args.file)?)?;
    let mode: GlobalMode = Default::default();

    // For each channel the first (or .0) is the sending half and the second
    // (or .1) is the receiving half
    let terminal_event_channel = mpsc::channel(16);
    let request_channel = mpsc::channel(32);

    let event_hangup_request_channel = mpsc::channel(2);

    let input_task = tokio::spawn(event::read_events(
        terminal_event_channel.0,
        event_hangup_request_channel.1,
    ));
    let request_task = tokio::spawn(input::process_request(
        request_channel.0,
        terminal_event_channel.1,
        mode.clone(),
    ));
    let render_task = tokio::spawn(ui::render_screen(
        request_channel.1,
        event_hangup_request_channel.0,
        mode,
    ));

    render_task.await.unwrap();
    input_task.await.unwrap();
    request_task.await.unwrap();

    save_luma(args.file).await;

    Ok(())
}

async fn init_logger(file: Option<PathBuf>) -> Result<()> {
    let log_file = file.unwrap_or_else(|| {
        PathBuf::from(env::var("XDG_CACHE_HOME").unwrap_or_else(|_| {
            let home = env::var("HOME").expect("You don't have a $HOME???");
            format!("{home}/.cache")
        }))
        .join("luma.log")
    });

    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        fs::File::create(log_file)?,
    )
    .unwrap();

    log::debug!("log init");
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
