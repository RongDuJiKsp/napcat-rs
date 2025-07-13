use crate::config::EmojiAttackConfig;
use kovi::event::GroupMsgEvent;
use kovi::log::error;
use kovi::RuntimeBot;
use kovi_plugin_expand_napcat::NapCatApi;
use std::sync::Arc;

pub async fn handle_group_msg(e: Arc<GroupMsgEvent>, bot: Arc<RuntimeBot>) {
    if !EmojiAttackConfig::get()
        .allow_monkey_groups
        .contains(&e.group_id)
    {
        return;
    }

    if let Err(e) = bot
        .set_msg_emoji_like(e.message_id as i64, EmojiAttackConfig::get().emoji.as_str())
        .await
    {
        error!("Failed to set message emoji literally: {}", e);
    }
}
