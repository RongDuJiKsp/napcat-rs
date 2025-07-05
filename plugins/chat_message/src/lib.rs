mod command_exec;
mod config;
mod handlers;
mod ml;

use crate::command_exec::common::register_common_cmd;
use crate::config::ChatConfigContext;
use kovi::log::error;
use kovi::{MsgEvent, PluginBuilder as plugin, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
async fn main() {
    app().await;
}
async fn app() {
    let bot = plugin::get_runtime_bot();
    ChatConfigContext::init(&bot)
        .await
        .expect("error on load ChatConfigContext");
    register_common_cmd().await;
    plugin::on_msg(move |event| on_group_msg(bot.clone(), event))
}
async fn on_group_msg(bot: Arc<RuntimeBot>, event: Arc<MsgEvent>) {
    if !event.is_group() {
        return;
    }
    if let Err(error) = handlers::group_chat::handle_group_chat(bot, event).await {
        error!("{error}")
    }
}
