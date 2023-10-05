//! A Program to unite the web and filesystem

#![warn(unused_crate_dependencies)]
// #![warn(missing_docs)]

pub mod app;
pub mod cli;
pub mod error;
pub mod event;
pub mod input;
pub mod prelude;
pub mod state;
pub mod ui;
#[macro_use]
pub mod util;

use std::path::{Path, PathBuf};

use crate::prelude::*;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = crate::cli::Args::parse();

    if args.log {
        init_logger(args.log_file).await?;
    }
    log::error!("Download dir: {}", DOWNLOAD_DIR);

    let file_str = args.file.to_str().unwrap().to_string();

    // TODO: use json::from_reader
    *LUMA.write().await = json::from_str(&fs::read_to_string(&args.file)?)?;
    let mode: GlobalMode = Default::default();

    // For each channel the first (or .0) is the sending half and the second
    // (or .1) is the receiving half
    let terminal_event_channel = mpsc::channel(16);
    let request_channel = mpsc::channel(32);
    let save_channel = mpsc::channel(2);

    let event_hangup_request_channel = mpsc::channel(2);
    let save_hangup_request_channel = mpsc::channel(2);

    let input_task = tokio::spawn(event::read_events(
        terminal_event_channel.0,
        event_hangup_request_channel.1,
    ));
    let request_task = tokio::spawn(input::process_request(
        request_channel.0,
        terminal_event_channel.1,
        mode.clone(),
    ));
    let save_thread = tokio::spawn(process_saves(
        save_channel.1,
        save_hangup_request_channel.1,
        args.file,
    ));

    ui::render_screen(
        request_channel.1,
        event_hangup_request_channel.0,
        save_channel.0,
        mode,
    )
    .await;

    input_task.await?;
    request_task.await?;

    save_hangup_request_channel
        .0
        .send(ThreadMessage::Close)
        .await
        .unwrap();
    save_thread.await?;

    if STATE.read().await.unsaved_changes {
        let q = requestty::Question::confirm("Save")
            .message("Save?")
            .default(true)
            .build();

        let Ok(requestty::Answer::Bool(save)) = requestty::prompt_one(q) else {
            return Err(LumaError::Generic("Question ask failed".into()));
        };
        if save {
            save_luma(file_str).await?
        }
    }

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

async fn process_saves(
    mut p_rx: mpsc::Receiver<Option<PathBuf>>,
    mut msgs: mpsc::Receiver<ThreadMessage>,
    default: PathBuf,
) {
    loop {
        tokio::select! {
            p = p_rx.recv() => {
                if let Some(Some(path)) = p {
                    save_luma(path).await.unwrap();
                } else {
                    save_luma(&default).await.unwrap();
                }
                STATE.write().await.unsaved_changes = false;
            },
            m = msgs.recv() => {
                if let Some(ThreadMessage::Close) = m {
                    break;
                }
            }
        }
    }
}

async fn save_luma(path: impl AsRef<Path>) -> Result<()> {
    let f = fs::File::create(path)?;
    json::to_writer_pretty::<_, Luma>(f, &LUMA.read().await as &Luma)?;
    Ok(())
}
