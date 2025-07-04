use crate::config::ChatConfigContext;
use kovi::tokio::time::sleep;
use kovi::MsgEvent;
use std::sync::Arc;
use std::time::Duration;

pub fn reply_as_im(ev: Arc<MsgEvent>, reply: &str) {
    let quotes = reply
        .split(&ChatConfigContext::get().model.dot_wait_tag)
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    kovi::spawn(async move {
        for reply in quotes.into_iter().filter(|x| x.len() > 0) {
            ev.reply(reply);
            sleep(Duration::from_millis(
                ChatConfigContext::get().model.dot_wait_time_ms,
            ))
            .await;
        }
    });
}
