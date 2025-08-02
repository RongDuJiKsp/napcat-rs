mod config;
mod data;
mod handle;

use crate::config::BanConfig;
use crate::data::BanData;
use kovi::event::GroupMsgEvent;
use kovi::log::{error, info};
use kovi::{PluginBuilder as plugin, RequestEvent, RuntimeBot};
use std::sync::Arc;

#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    BanConfig::init(&bot).expect("Failed to initialize BanDynConfig");
    BanData::init(&bot).expect("Failed to initialize BanData");
    let bot1 = bot.clone();
    let bot2 = bot.clone();
    plugin::on_group_msg(move |e| on_msg(e, bot1.clone()));
    plugin::on_request(move |e| on_request(e, bot2.clone()));
}
async fn on_msg(e: Arc<GroupMsgEvent>, bot: Arc<RuntimeBot>) {
    if let Err(error) = handle::on_chat(e, bot).await {
        error!("{:?}", error);
    }
}
async fn on_request(e: Arc<RequestEvent>, bot: Arc<RuntimeBot>) {
    info!("接收到群邀请请求：{:?}", e);
    if let Err(error) = handle::on_request(e, bot).await {
        error!("{:?}", error);
    }
}
