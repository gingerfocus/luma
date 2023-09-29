pub mod key;
pub mod mouse;

use std::time::Duration;

use crossterm::event::Event as CrossEvent;

use crate::prelude::*;

use self::{key::Key, mouse::Mouse};

macro_rules! protocal_error {
    ($msg:expr, $state:expr) => {
        panic!(
            "Invalid msg received ({}) for current state ({}).",
            $msg, $state
        )
    };
}

/// The main task that is spawned to read the events
pub async fn read_events(
    tx: mpsc::Sender<Event>,
    mut hangup_requests: mpsc::Receiver<ThreadMessage>,
) {
    log::info!("event thread created");

    let mut reader = crossterm::event::EventStream::new();
    let mut paused = false;
    let mut run = true;

    while run {
        if paused {
            log::debug!("waiting for pause to end");
            tokio::select! {
                e = hangup_requests.recv() => {
                    match e {
                        Some(ThreadMessage::Resume) => {
                            paused = false;
                            // HACK: When returning from running an external
                            // program/there is a phantom event that is read 
                            // (immediatly?). This next call to the reader just
                            // discards that message which can often be bad
                            let _ = futures::StreamExt::next(&mut reader).await;
                        },
                        Some(ThreadMessage::Close) => run = false,
                        Some(ThreadMessage::Pause(_)) => protocal_error!("pause", "paused"),
                        None => {},
                    }
                },
                _ = tx.closed() => {
                    log::warn!("event consumer gone, reader exiting");
                    run = false;
                },
            };
        } else {
            log::trace!("running poll routine");
            let delay = tokio::time::sleep(Duration::from_secs(1));
            let event_steam = futures::StreamExt::next(&mut reader);

            let mut event = None;
            tokio::select! {
                e = hangup_requests.recv() => {
                    match e {
                        Some(ThreadMessage::Pause(resp)) => {
                            resp.send(ThreadMessageResponse::Ok).unwrap();
                            paused = true;
                        }
                        Some(ThreadMessage::Close) => run = false,
                        Some(ThreadMessage::Resume) => protocal_error!("resume", "running"),
                        None => {},
                    }
                },
                _ = tx.closed() => run = false,
                _ = delay => event = Some(Event::Tick),
                e = event_steam => {
                    event = e.and_then(|res| res.ok().map(|ev| ev.into()));
                },
            };

            if let Some(ev) = event {
                if tx.send(ev).await.is_err() {
                    run = false;
                }
            }
        }
    }

    log::info!("event thread ended");
}

/// An occurred event.
#[derive(Default, Debug)]
pub enum Event {
    /// An input event occurred.
    Input(Key),
    /// When the mouse is clicked
    Click(Mouse),
    /// when the terminal is resized
    Resize(u16, u16),
    /// A change in focus for the screen, true when gained, false when lost
    GainedFocus(bool),
    /// An tick event occurred.
    #[default]
    Tick,
}

impl From<CrossEvent> for Event {
    fn from(value: crossterm::event::Event) -> Self {
        match value {
            CrossEvent::FocusGained => Event::GainedFocus(true),
            CrossEvent::FocusLost => Event::GainedFocus(false),
            CrossEvent::Key(ke) => Event::Input(ke.into()),
            CrossEvent::Mouse(me) => Event::Click(me.into()),
            CrossEvent::Paste(_) => unimplemented!(),
            CrossEvent::Resize(x, y) => Event::Resize(x, y),
        }
    }
}
