pub mod key;
pub mod mouse;

use std::time::Duration;

use crossterm::event::Event as CrossEvent;
use tokio::sync::oneshot;

use crate::prelude::*;

use self::{key::Key, mouse::Mouse};

/// The main task that is spawned to read the events
pub async fn read_events(
    tx: mpsc::Sender<Event>,
    mut hangup_requests: mpsc::Receiver<EventReaderRequest>,
) {
    log::info!("event thread created");

    let mut screen = Screen::default();
    screen.init().unwrap();

    let mut reader = crossterm::event::EventStream::new();
    let mut paused = false;
    let mut run = true;

    while run {
        let delay = tokio::time::sleep(Duration::from_secs(1));
        let event_steam = futures::StreamExt::next(&mut reader);

        let mut event = None;

        tokio::select! {
            e = hangup_requests.recv() => {
                match e {
                    Some(EventReaderRequest::Pause(resp)) => {
                        resp.send(EventThreadSuspendResponse::Ok).unwrap();
                        screen.deinit().unwrap();
                        paused = true;
                    }
                    Some(EventReaderRequest::Resume) => {
                        screen.init().unwrap();
                        paused = false;
                    }
                    Some(EventReaderRequest::Close) => run = false,
                    None => {},
                }
            },
            _ = tx.closed() => run = false,
            _ = delay, if !paused => event = Some(Event::Tick),
            e = event_steam, if !paused => event = e.and_then(|res| res.ok().map(|ev| ev.into())),
        };

        if let Some(ev) = event {
            if tx.send(ev).await.is_err() {
                run = false;
            }
        }
    }

    screen.deinit().unwrap();
    log::info!("event thread ended");
}

pub enum EventReaderRequest {
    Pause(oneshot::Sender<EventThreadSuspendResponse>),
    Resume,
    Close,
}

#[derive(Debug)]
pub enum EventThreadSuspendResponse {
    Ok, // (oneshot::Sender<EventReaderRequest>),
    No(String),
    Err(LumaError),
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
    /// when some text is put into the terminal
    Paste(String),
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
            CrossEvent::Paste(s) => Event::Paste(s),
            CrossEvent::Resize(x, y) => Event::Resize(x, y),
        }
    }
}
