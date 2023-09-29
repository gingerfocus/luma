pub mod render;
pub mod screen;
pub mod traits;

use std::{io::Write, time::Duration};

use tokio::task::JoinHandle;

use crate::prelude::*;

pub async fn render_screen(
    mut rx: mpsc::Receiver<LumaMessage>,
    event_tx: mpsc::Sender<crate::event::EventReaderRequest>,
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
            handle_msg(msg, &mut app, &mode, &event_tx, &mut handles).await;
        }

        let mut new_handles = vec![];
        for h in handles {
            if h.is_finished() {
                if let Ok(msgs) = h.await {
                    for msg in msgs {
                        handle_msg(msg, &mut app, &mode, &event_tx, &mut new_handles).await;
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
    event_tx: &mpsc::Sender<crate::event::EventReaderRequest>,
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
            log::debug!("Renderer opening editor");

            let (tx, rx) = oneshot::channel();
            event_tx
                .send(crate::event::EventReaderRequest::Pause(tx))
                .await
                .unwrap();

            app.deinit().unwrap();

            // reader needs some time to ready the terminal so we wait a bit
            tokio::time::sleep(Duration::from_millis(2)).await;

            if let Some(msg) = match rx.await {
                Ok(crate::event::EventThreadSuspendResponse::Ok) => None,
                Ok(crate::event::EventThreadSuspendResponse::No(reason)) => Some(format!(
                    "Thread refused hangup request with msg: {}",
                    reason
                )),
                Ok(crate::event::EventThreadSuspendResponse::Err(e)) => {
                    Some(format!("Thread failed hangup request with err: {}", e))
                }
                Err(e) => Some(format!("Recv Errored: {}", e)),
            } {
                log::error!("{}", msg);
                panic!("{}", msg);
            }

            log::debug!("paused event channel sucsessfully");

            const LUMA_TEMPFILE: &str = "/tmp/luma-edit.yaml";
            {
                let mut f = fs::File::create(LUMA_TEMPFILE).unwrap();
                f.write_all(text.as_bytes()).unwrap();
                log::debug!("wrote inital data to buffer");
            }

            let _ = tokio::process::Command::new("nvim")
                .arg(LUMA_TEMPFILE)
                .spawn()
                .unwrap()
                .wait()
                .await
                .unwrap()
                .success();

            log::debug!("asked question");

            let res = fs::read(LUMA_TEMPFILE).unwrap();

            app.init().unwrap();

            // event thream reads phantom events so giving it some breathing room helps
            tokio::time::sleep(Duration::from_millis(2)).await;

            event_tx
                .send(crate::event::EventReaderRequest::Resume)
                .await
                .unwrap();

            app.redraw(&mode.read().unwrap()).unwrap();

            if let Ok(ans) = String::from_utf8(res) {
                resp.send(ans).unwrap();
            }
        }
        LumaMessage::AddHandle(h) => handles.push(h),
    }
}
