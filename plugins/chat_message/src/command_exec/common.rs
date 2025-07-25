use crate::config::{ChatConfig, SyncControl};
use crate::handlers::group_chat::NyaCatMemory;
use crate::handlers::tool::reply_as_im;
use crate::ml;
use kovi_plugin_command_exec::app::{BotCommand, BotCommandBuilder};

pub async fn register_common_cmd() {
    BotCommandBuilder::on_common_command("$smart", |e| exec_smart(e)).await;
    BotCommandBuilder::on_common_command("$hi", |e| exec_hi(e)).await;
    BotCommandBuilder::on_super_command("$restart", |e| exec_live(e)).await;
    BotCommandBuilder::on_super_command("$kill", |e| exec_kill(e)).await;
    BotCommandBuilder::on_super_command("$mem_kill", |e| exec_mem_kill(e)).await;
}

async fn exec_hi(e: BotCommand) {
    if !ok_exec(&e) {
        return;
    }
    if !SyncControl::running() {
        return;
    }
    e.event
        .reply_and_quote("你好喵！我是一只猫娘喵！前面忘了中间忘了，反正我是一只猫娘喵")
}
async fn exec_kill(e: BotCommand) {
    if !ok_exec(&e) {
        return;
    }
    if !SyncControl::running() {
        return;
    }
    e.event.reply_and_quote("猫娘似了喵");
    SyncControl::set_bot_run(false);
}
async fn exec_live(e: BotCommand) {
    if !ok_exec(&e) {
        return;
    }
    if SyncControl::running() {
        return;
    }
    e.event.reply_and_quote("猫娘复活了喵");
    SyncControl::set_bot_run(true);
}
async fn exec_smart(e: BotCommand) {
    if !ok_exec(&e) {
        return;
    }
    if !SyncControl::running() {
        return;
    }
    let q = e.args.join(" ");
    if !q.is_empty() {
        reply_as_im(
            e.event.clone(),
            &ml::get_reply_as_smart_nya_cat(&q)
                .await
                .unwrap_or_else(|e| format!("发生错误了喵：{}", e)),
        )
    } else {
        e.event.reply_and_quote("聪明猫娘在这里喵！");
    }
}
async fn exec_mem_kill(e: BotCommand) {
    if !ok_exec(&e) {
        return;
    }
    NyaCatMemory::load().write().await.clean();
    e.event.reply_and_quote("猫娘的记忆被抹除成功了喵！");
}
fn ok_exec(e: &BotCommand) -> bool {
    e.event
        .group_id
        .map(|id| ChatConfig::get().allow_groups.contains(&id))
        .unwrap_or(false)
}
