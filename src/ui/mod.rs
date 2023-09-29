pub mod render;
pub mod screen;
pub mod traits;

use std::time::Duration;

use tokio::task::JoinHandle;

use crate::{prelude::*, util::programs::Programs};

pub async fn render_screen(
    mut rx: mpsc::Receiver<LumaMessage>,
    event_thread_channel: mpsc::Sender<ThreadMessage>,
    mode: GlobalMode,
) {
    log::info!("render thread created");

    let mut handles: Vec<JoinHandle<Vec<LumaMessage>>> = vec![];
    let mut app = App::new().unwrap();
    app.init().unwrap();

    // --------------------------------------------
    app.draw(&mode.read().unwrap()).unwrap();

    while app.run {
        if let Some(msg) = rx.recv().await {
            handle_msg(msg, &mut app, &mode, &event_thread_channel, &mut handles).await;
        }

        let mut new_handles = vec![];
        for h in handles {
            if h.is_finished() {
                if let Ok(msgs) = h.await {
                    for msg in msgs {
                        handle_msg(
                            msg,
                            &mut app,
                            &mode,
                            &event_thread_channel,
                            &mut new_handles,
                        )
                        .await;
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

async fn handle_msg(
    msg: LumaMessage,
    app: &mut App,
    mode: &GlobalMode,
    event_tx: &mpsc::Sender<ThreadMessage>,
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
        LumaMessage::OpenEditor { text, resp } => {
            log::debug!("Opening editor");

            pause_event_channel(app, event_tx).await.unwrap();

            Programs::editor(text, resp).await.run().await.unwrap();

            resume_event_channel(app, event_tx).await.unwrap();

            app.redraw(&mode.read().unwrap()).unwrap();
        }
        LumaMessage::AddHandle(h) => handles.push(h),
    }
}

async fn pause_event_channel(app: &mut App, event_tx: &mpsc::Sender<ThreadMessage>) -> Result<()> {
    let (tx, rx) = oneshot::channel();
    event_tx.send(ThreadMessage::Pause(tx)).await?;

    app.deinit()?;

    // reader needs some time to ready the terminal so we wait a bit
    tokio::time::sleep(Duration::from_millis(2)).await;

    if let Some(msg) = match rx.await {
        Ok(ThreadMessageResponse::Ok) => None,
        Ok(ThreadMessageResponse::No(reason)) => Some(format!(
            "Thread refused hangup request with msg: {}",
            reason
        )),
        Ok(ThreadMessageResponse::Err(e)) => {
            Some(format!("Thread failed hangup request with err: {}", e))
        }
        Err(e) => Some(format!("Recv Errored: {}", e)),
    } {
        log::error!("{}", msg);
        panic!("{}", msg);
    }

    log::debug!("paused event channel sucsessfully");
    Ok(())
}

async fn resume_event_channel(app: &mut App, event_tx: &mpsc::Sender<ThreadMessage>) -> Result<()> {
    app.init()?;

    // event thream reads phantom events so giving it some breathing room helps
    tokio::time::sleep(Duration::from_millis(2)).await;

    event_tx.send(ThreadMessage::Resume).await.unwrap();
    Ok(())
}
