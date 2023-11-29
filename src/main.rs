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
    let args = cli::Args::parse();

    if args.log {
        init_logger(args.log_file)?;
        log::info!("log init");
    }

    let f = fs::File::open(&args.file)?;
    *LUMA.write().await = json::from_reader(f)?;

    let _mode = Mode::default();

    crate::ui::task::render().await;

    if STATE.read().await.unsaved_changes {
        let q = requestty::Question::confirm("Save")
            .message("Save?")
            .default(true)
            .build();

        let Ok(requestty::Answer::Bool(save)) = requestty::prompt_one(q) else {
            return Err("Question ask failed".into());
        };

        if save {
            save_luma(&args.file).await?
        }
    }

    Ok(())
}

fn init_logger(file: Option<PathBuf>) -> Result<()> {
    let log_file = file.unwrap_or_else(|| {
        PathBuf::from(env::var("XDG_CACHE_HOME").unwrap_or_else(|_| {
            let home = env::var("HOME").expect("You don't have a $HOME???");
            format!("{home}/.cache")
        }))
        .join("luma.log")
    });

    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Trace,
        simplelog::Config::default(),
        fs::File::create(log_file)?,
    )
    .unwrap();

    log::debug!("log init");
    Ok(())
}

async fn save_luma(path: impl AsRef<Path>) -> Result<()> {
    let f = fs::File::create(path)?;
    json::to_writer_pretty::<_, Luma>(f, &LUMA.read().await as &Luma)?;
    Ok(())
}
