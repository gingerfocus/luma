use std::time::Duration;
use crate::prelude::*;

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
                        Some(ThreadMessage::Pause(_)) => panic!("Invalid msg: Received (pause) which is invalid for current state (pause)."),
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
                        Some(ThreadMessage::Resume) => panic!("Invalid msg: Received (resume) which is invalid for current state (running)."),
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
