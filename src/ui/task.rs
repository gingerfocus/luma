use crate::prelude::*;
use tuirealm::{PollStrategy, Update};

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
