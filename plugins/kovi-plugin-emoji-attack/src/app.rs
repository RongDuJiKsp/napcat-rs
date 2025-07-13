use crate::config::EmojiAttackConfig;
use kovi::PluginBuilder as plugin;
use kovi::bot::runtimebot::RuntimeBot;

use kovi::tokio::sync::RwLock;

use std::collections::HashSet;
use std::sync::{Arc, OnceLock};
use crate::handle::handle_group_msg;

static EMOJI_ALL_USER: OnceLock<RwLock<HashSet<i64>>> = OnceLock::new();
pub async fn init() {
    let bot: Arc<RuntimeBot> = plugin::get_runtime_bot();
    EMOJI_ALL_USER
        .set(RwLock::new(HashSet::new()))
        .expect("init EMOJI_ALL_USER failed");
    EmojiAttackConfig::init(&bot).await.unwrap();
    plugin::on_group_msg(move |e| handle_group_msg(e, bot.clone()))
}
