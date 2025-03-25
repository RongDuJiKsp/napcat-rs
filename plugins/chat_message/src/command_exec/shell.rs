use crate::command_exec::app::{BotCommand, BotCommandBuilder};
use std::sync::atomic::{AtomicUsize, Ordering};
use kovi::log::info;

pub async fn register_shell_cmd() {
    BotCommandBuilder::on_super_command("$shell", |e| exec_shell_cmd(e)).await;
    BotCommandBuilder::on_common_command("$hi", |e| exec_hi(e)).await;
}
static ID: AtomicUsize = AtomicUsize::new(1156);
async fn exec_shell_cmd(e: BotCommand) {
    info!("Shell Called");
    e.event.reply_and_quote(format!("shell创建成功喵！编号 {}", ID.fetch_add(1, Ordering::Relaxed)))
}
async fn exec_hi(e: BotCommand) {
    e.event.reply_and_quote("你好喵！我是一只猫娘喵！前面忘了中间忘了，反正我是一只猫娘喵")
}