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
    for cmd in e
        .message
        .get("text")
        .iter()
        .filter_map(|e| e.data.get("text"))
        .filter_map(|v| v.as_str())
        .filter(|str| str.starts_with("$"))
    {
        BotCommand::from_str(cmd, e.clone()).invoke_command().await;
    }
}
