use tuirealm::{PollStrategy, Update};
use crate::prelude::*;

pub async fn render() {
    log::info!("render thread created");

    // let mut handles: Vec<LumaTask> = vec![];
    // let mut app = App::new().unwrap();
    let mut model = Model::new().unwrap();

    // --------------------------------------------
    let _ = model.terminal.enable_raw_mode();
    let _ = model.terminal.enter_alternate_screen();

    while !model.quit {
        match model.app.tick(PollStrategy::Once) {
            Ok(msgs) if msgs.len() > 0 => {
                model.redraw = true;

                for msg in msgs {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }
        model.view();
        // if model.redraw {
        //     model.view();
        //     model.redraw = false;
        // }
    }

    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    // -----------------------------------------------

    // while app.run {
    //     if let Some(msg) = rx.recv().await {
    //         handle_msg(
    //             msg,
    //             &mut app,
    //             &mode,
    //             &event_thread_channel,
    //             &save_thread_channel,
    //             &mut handles,
    //         )
    //         .await;
    //     }
    //
    //     let mut new_handles = vec![];
    //     for h in handles {
    //         if h.1.is_finished() {
    //             if let Ok(msgs) = h.1.await {
    //                 for msg in msgs {
    //                     handle_msg(
    //                         msg,
    //                         &mut app,
    //                         &mode,
    //                         &event_thread_channel,
    //                         &save_thread_channel,
    //                         &mut new_handles,
    //                     )
    //                     .await;
    //                 }
    //             }
    //         } else {
    //             new_handles.push(h);
    //         }
    //     }
    //     handles = new_handles;
    //
    //     // if let Ok(Some(msg)) = block_on(h) {
    //     //     handle_msg(msg, &mut app, &mode, &mut handles);
    //     // }
    //     // None
    // }
    // --------------------------------------------

    log::info!("render thread ended");
}

// async fn handle_msg(
//     msg: LumaMessage,
//     app: &mut App,
//     mode: &GlobalMode,
//     event_tx: &mpsc::Sender<ThreadMessage>,
//     save_tx: &mpsc::Sender<Option<PathBuf>>,
//     handles: &mut Vec<LumaTask>,
// ) {
//     match msg {
//         LumaMessage::Redraw => {
//             app.draw(&mode.read().await as &Mode).unwrap();
//         }
//         LumaMessage::Exit => {
//             app.run = false;
//         }
//         LumaMessage::SetMode(m) => {
//             log::info!("waiting on write for mode.");
//             *mode.write().await = m;
//             app.draw(&mode.read().await as &Mode).unwrap();
//         }
//         LumaMessage::OpenEditor { text, resp } => {
//             log::debug!("Opening editor");
//
//             // pause_event_channel(app, event_tx).await.unwrap();
//
//             Programs::editor(text, resp).await.run().await.unwrap();
//
//             // resume_event_channel(app, event_tx).await.unwrap();
//
//             app.redraw(&mode.read().await as &Mode).unwrap();
//         }
//         LumaMessage::AddHandle(h) => handles.push(h),
//         LumaMessage::Save(p) => save_tx.send(p).await.unwrap(),
//     }
// }

// async fn pause_event_channel(app: &mut App, event_tx: &mpsc::Sender<ThreadMessage>) -> Result<()> {
//     let (tx, rx) = oneshot::channel();
//     event_tx.send(ThreadMessage::Pause(tx)).await?;
//
//     app.deinit()?;
//
//     // reader needs some time to ready the terminal so we wait a bit
//     tokio::time::sleep(Duration::from_millis(2)).await;
//
//     if let Some(msg) = match rx.await {
//         Ok(ThreadMessageResponse::Ok) => None,
//         Ok(ThreadMessageResponse::No(reason)) => Some(format!(
//             "Thread refused hangup request with msg: {}",
//             reason
//         )),
//         Ok(ThreadMessageResponse::Err(e)) => {
//             Some(format!("Thread failed hangup request with err: {}", e))
//         }
//         Err(e) => Some(format!("Recv Errored: {}", e)),
//     } {
//         log::error!("{}", msg);
//         panic!("{}", msg);
//     }
//
//     log::debug!("paused event channel sucsessfully");
//     Ok(())
// }
//
// async fn resume_event_channel(app: &mut App, event_tx: &mpsc::Sender<ThreadMessage>) -> Result<()> {
//     app.init()?;
//
//     // event thream reads phantom events so giving it some breathing room helps
//     tokio::time::sleep(Duration::from_millis(2)).await;
//
//     event_tx.send(ThreadMessage::Resume).await.unwrap();
//     Ok(())
// }
