use crate::command_exec::app::{BotCommand, BotCommandBuilder};
use crate::ml;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub async fn register_shell_cmd() {
    BotCommandBuilder::on_super_command("$shell", |e| exec_shell_cmd(e)).await;
    BotCommandBuilder::on_common_command("$smart", |e| exec_smart(e)).await;
    BotCommandBuilder::on_common_command("$hi", |e| exec_hi(e)).await;
    BotCommandBuilder::on_common_command("$restart", |e| exec_live(e)).await;
    BotCommandBuilder::on_common_command("$kill", |e| exec_kill(e)).await;
}
static ID: AtomicUsize = AtomicUsize::new(1156);
static LIVE: AtomicBool = AtomicBool::new(true);
async fn exec_shell_cmd(e: BotCommand) {
    if !LIVE.load(Ordering::Relaxed) {
        return;
    }
    e.event.reply_and_quote(format!(
        "shell创建成功喵！编号 {}",
        ID.fetch_add(1, Ordering::Relaxed)
    ))
}
async fn exec_hi(e: BotCommand) {
    if !LIVE.load(Ordering::Relaxed) {
        return;
    }
    e.event
        .reply_and_quote("你好喵！我是一只猫娘喵！前面忘了中间忘了，反正我是一只猫娘喵")
}
async fn exec_kill(e: BotCommand) {
    if !LIVE.load(Ordering::Relaxed) {
        return;
    }
    e.event.reply_and_quote("似了喵");
    LIVE.store(false, Ordering::Relaxed)
}
async fn exec_live(e: BotCommand) {
    if LIVE.load(Ordering::Relaxed) {
        return;
    }
    e.event.reply_and_quote("复活了喵");
    LIVE.store(true, Ordering::Relaxed)
}
async fn exec_smart(e: BotCommand) {
    if let Some(q) = e.args.get(0) {
        e.event.reply_and_quote(ml::get_reply_as_smart_nya_cat(q).await.unwrap_or_else(|e| format!("发生错误了喵：{}", e)))
    } else {
        e.event.reply_and_quote("聪明猫娘在这里喵！");
    }
}