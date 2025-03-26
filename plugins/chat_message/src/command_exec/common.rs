use crate::command_exec::app::{BotCommand, BotCommandBuilder};
use crate::config::SyncControl;
use crate::handlers::group_chat::NyaCatMemory;
use crate::ml;
use std::sync::atomic::{AtomicUsize, Ordering};

pub async fn register_common_cmd() {
    BotCommandBuilder::on_super_command("$shell", |e| exec_shell_cmd(e)).await;
    BotCommandBuilder::on_common_command("$smart", |e| exec_smart(e)).await;
    BotCommandBuilder::on_common_command("$hi", |e| exec_hi(e)).await;
    BotCommandBuilder::on_super_command("$restart", |e| exec_live(e)).await;
    BotCommandBuilder::on_super_command("$kill", |e| exec_kill(e)).await;
    BotCommandBuilder::on_super_command("$mem_kill", |e| exec_mem_kill(e)).await;
}
static ID: AtomicUsize = AtomicUsize::new(1156);
async fn exec_shell_cmd(e: BotCommand) {
    if !SyncControl::running() {
        return;
    }
    e.event.reply_and_quote(format!(
        "shell创建成功喵！编号 {}",
        ID.fetch_add(1, Ordering::Relaxed)
    ))
}
async fn exec_hi(e: BotCommand) {
    if !SyncControl::running() {
        return;
    }
    e.event
        .reply_and_quote("你好喵！我是一只猫娘喵！前面忘了中间忘了，反正我是一只猫娘喵")
}
async fn exec_kill(e: BotCommand) {
    if !SyncControl::running() {
        return;
    }
    e.event.reply_and_quote("猫娘似了喵");
    SyncControl::set_bot_run(false);
}
async fn exec_live(e: BotCommand) {
    if SyncControl::running() {
        return;
    }
    e.event.reply_and_quote("猫娘复活了喵");
    SyncControl::set_bot_run(true);
}
async fn exec_smart(e: BotCommand) {
    if !SyncControl::running() {
        return;
    }
    if let Some(q) = e.args.get(0) {
        e.event.reply_and_quote(
            ml::get_reply_as_smart_nya_cat(q)
                .await
                .unwrap_or_else(|e| format!("发生错误了喵：{}", e)),
        )
    } else {
        e.event.reply_and_quote("聪明猫娘在这里喵！");
    }
}
async fn exec_mem_kill(e: BotCommand) {
    NyaCatMemory::load().write().await.clean();
    e.event.reply_and_quote("猫娘的记忆被抹除成功了喵！");
}
