use crate::config::EmojiAttackConfig;
use crate::data::EmojiAttackData;
use kovi::event::GroupMsgEvent;
use kovi::log::error;
use kovi::tokio::time::sleep;
use kovi::RuntimeBot;
use kovi_plugin_command_exec::app::{BotCommand, BotCommandBuilder};
use kovi_plugin_expand_napcat::NapCatApi;
use std::collections::HashSet;
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
async fn handle_cmd(e: BotCommand) {
    let group_id = match e.event.group_id {
        Some(x) => x,
        None => return,
    };
    let command = e.args.get(0).unwrap_or(&NULL_STR).as_str();
    if HashSet::from(["add", "del", "clean"]).contains(command) {
        handle_auto_cmd(&e, command, group_id).await;
        return;
    }
}
async fn handle_auto_cmd(e: &BotCommand, cmd: &str, group_id: i64) {
    let targets = {
        let mut targets = e
            .event
            .message
            .get("at")
            .iter()
            .filter_map(|s| s.data.get("qq"))
            .filter_map(|v| v.as_str())
            .filter_map(|s| s.parse::<i64>().ok())
            .filter(|x| *x != e.event.self_id)
            .collect::<Vec<_>>();
        if e.args.len() > 1 {
            targets.append(
                &mut e.args[1..]
                    .iter()
                    .filter_map(|x| x.parse::<i64>().ok())
                    .collect::<Vec<_>>(),
            );
        }
        targets
    };
    let data = EmojiAttackData::get();
    let mut lock = data.write().await;

    let result = match cmd {
        "add" => targets
            .iter()
            .map(|target| {
                lock.group_users
                    .entry(group_id)
                    .or_default()
                    .insert(*target)
            })
            .any(|x| x),
        "del" => targets
            .iter()
            .map(|target| lock.group_users.entry(group_id).or_default().remove(target))
            .any(|x| x),
        "clean" => {
            lock.group_users.entry(group_id).or_default().clear();
            true
        }
        _ => {
            error!("存在处理器处理不了的命令");
            false
        }
    };
    e.event
        .reply(format!("操作{}喵！", if result { "成功" } else { "失败" }));
}
pub async fn register_cmd() {
    BotCommandBuilder::on_super_command("$monkey", |e| handle_cmd(e)).await;
}
