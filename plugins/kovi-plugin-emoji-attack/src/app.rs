use crate::config::EmojiAttackConfig;
use kovi::PluginBuilder as plugin;
use kovi::tokio::sync::RwLock;
use kovi_plugin_expand_napcat::NapCatApi;
use std::collections::HashSet;
use std::sync::{Arc, OnceLock};
use kovi::bot::runtimebot::RuntimeBot;
static EMOJI_ALL_USER: OnceLock<RwLock<HashSet<i64>>> = OnceLock::new();
pub async fn init() {
    let bot:Arc<RuntimeBot> = plugin::get_runtime_bot();
    EMOJI_ALL_USER
        .set(RwLock::new(HashSet::new()))
        .expect("init EMOJI_ALL_USER failed");
    EmojiAttackConfig::init(&bot).await.unwrap();
    plugin::on_group_msg(|e| async move {
        if !e.is_group() {
            return;
        }
        if !EmojiAttackConfig::get()
            .allow_monkey_groups
            .contains(&e.group_id.unwrap())
        {
            return;
        }
        <RuntimeBot as NapCatApi>::set_msg_emoji_like(
            &*bot,
            e.message_id as i64,
            EmojiAttackConfig::get().emoji.as_str(),
        );
    })
}
