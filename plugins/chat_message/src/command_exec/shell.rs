use crate::command_exec::app::{BotCommand, BotCommandBuilder};
use crate::config::SyncControl;
use std::sync::atomic::{AtomicUsize, Ordering};

async fn register_shell_cmd() {
    BotCommandBuilder::on_super_command("$shell", |e| exec_shell_cmd(e)).await;
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