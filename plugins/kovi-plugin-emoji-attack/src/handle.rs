use crate::config::EmojiAttackConfig;
use crate::data::EmojiAttackData;
use kovi::event::GroupMsgEvent;
use kovi::log::error;
use kovi::tokio::time::sleep;
use kovi::RuntimeBot;
use kovi_plugin_command_exec::app::{BotCommand, BotCommandBuilder};
use kovi_plugin_expand_napcat::NapCatApi;
use std::sync::Arc;
use std::time::Duration;

static NULL_STR: String = String::new();
pub async fn handle_group_msg(e: Arc<GroupMsgEvent>, bot: Arc<RuntimeBot>) {
    if !EmojiAttackConfig::get()
        .allow_monkey_groups
        .contains(&e.group_id)
    {
        return;
    }
    if !EmojiAttackData::get()
        .read()
        .await
        .group_users
        .get(&e.group_id)
        .map(|s| s.contains(&e.user_id))
        .unwrap_or(false)
    {
        return;
    }
    let c = EmojiAttackConfig::get();
    for ji in &c.emoji {
        if let Err(e) = bot
            .set_msg_emoji_like(e.message_id as i64, ji.as_str())
            .await
        {
            error!("Failed to set message emoji literally: {}", e);
        }
        sleep(Duration::from_millis(c.wait_ms.unwrap_or(300))).await;
    }
}
pub async fn handle_cmd(e: BotCommand) {
    let group_id = match e.event.group_id {
        Some(x) => x,
        None => return,
    };
    let target = match e.args.get(1).and_then(|x| x.parse::<i64>().ok()) {
        Some(x) => x,
        None => {
            e.event.reply_and_quote("请给出指令目标！");
            return;
        }
    };
    let data = EmojiAttackData::get();
    let mut lock = data.write().await;

    let result = match e.args.get(0).unwrap_or(&NULL_STR).as_str() {
        "add" => lock.group_users.entry(group_id).or_default().insert(target),
        "del" => lock
            .group_users
            .entry(group_id)
            .or_default()
            .remove(&target),
        _ => false,
    };
    e.event
        .reply(format!("操作{}喵！", if result { "成功" } else { "失败" }));
}

pub async fn register_cmd() {
    BotCommandBuilder::on_super_command("$monkey", |e| handle_cmd(e)).await;
}
