use std::time::Duration;

use crate::{
    event::{Mouse, MouseKind},
    prelude::*,
};

// The main task that is spawn to process the key inputs into requests
// pub async fn process_request(
//     tx: mpsc::Sender<LumaMessage>,
//     mut rx: mpsc::Receiver<Event>,
//     mode: &mut Mode,
// ) {
//     log::info!("request thread created");
//     let mut handler = Handler::new();
//
//     loop {
//         let timeout = tokio::time::sleep(Duration::from_secs(2));
//
//         tokio::select! {
//             _ = tx.closed() => break,
//             _ = timeout => continue,
//             e = rx.recv() => {
//                 if let Some(event) = e {
//                     log::trace!("got key event: {:?}", event);
//
//                     for req in handle(event, &mut handler, mode).await {
//                         if tx.send(req).await.is_err() { break; }
//                     }
//                 }
//             }
//         }
//     }
//     log::info!("request thread ended");
// }
//
// pub async fn handle(event: Event, handler: &mut Handler, mode: &mut Mode) -> Vec<LumaMessage> {
//     match event {
//         Event::GainedFocus(_did) => vec![],
//         Event::Input(key) => handler.handle(key, mode).await,
//         Event::Click(click) => handle_click(click).await,
//         Event::Resize(_x, _y) => vec![LumaMessage::Redraw],
//         Event::Tick => vec![],
//         _ => vec![]
//     }
// }
//
// async fn handle_click(click: Mouse) -> Vec<LumaMessage> {
//     let Mouse { kind, .. } = click;
//     match kind {
//         MouseKind::LeftClick => todo!(),
//         MouseKind::RightClick => todo!(),
//         MouseKind::MiddleClick => todo!(),
//         MouseKind::Drag => todo!(),
//         MouseKind::Scroll(i) => {
//             let mut gwrite = STATE.write().await;
//             if i < 0 {
//                 _ = gwrite.selected_index.saturating_sub(-i as usize);
//             } else {
//                 gwrite.selected_index += i as usize;
//             }
//             vec![LumaMessage::Redraw]
//         }
//         MouseKind::Nothing => vec![],
//     }
// }
