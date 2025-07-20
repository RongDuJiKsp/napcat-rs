use crate::app::{BotCommand, GLOBAL_BOT};
use crate::config::CommandExecConfig;
use kovi::{MsgEvent, PluginBuilder as plugin};
use std::sync::Arc;

pub mod app;
pub mod config;
#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    CommandExecConfig::init(&bot).unwrap();
    GLOBAL_BOT
        .set(bot.clone())
        .map_err(|_e| anyhow::anyhow!("failed to set global bot"))
        .unwrap();
    plugin::on_msg(|msg| on_msg(msg));
}

async fn on_msg(e: Arc<MsgEvent>) {
    if let Some(cmd) = e
        .text
        .as_ref()
        .and_then(|e| if e.starts_with("$") { Some(e) } else { None })
    {
        BotCommand::from_str(cmd, e.clone()).invoke_command().await;
    }
}
