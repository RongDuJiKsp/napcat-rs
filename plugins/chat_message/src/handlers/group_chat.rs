use crate::command_exec::app::BotCommand;
use crate::config::ChatConfigContext;
use crate::tools;
use anyhow::anyhow;
use kovi::{MsgEvent, RuntimeBot};
use std::error::Error;
use std::sync::Arc;

pub async fn handle_group_chat(bot: Arc<RuntimeBot>, event: Arc<MsgEvent>) -> Result<(), Box<dyn Error>> {
    //只考虑已经监听的群
    if !ChatConfigContext::get().allow_groups.contains(&event.group_id.ok_or(anyhow!("找不到群id"))?) {
        return Ok(());
    }
    //有人@猫娘
    if event.message.contains("at") && event.message.get("at").get(0).and_then(|s| s.data.get("qq")).and_then(|v| v.as_str().and_then(|s| s.parse::<i64>().ok())).and_then(|e| if e == event.self_id { Some(()) } else { None }).is_some() {
        at_me(event.clone()).await;
        return Ok(());
    }
    let bot_info = tools::self_bot_info(&bot, &event).await.ok();
    //若有bot info
    if let Some(bot_if) = &bot_info {
        //判断消息是否有猫娘的名字
        if event.text.as_ref().and_then(|e| if e.contains(&bot_if.nickname) { Some(()) } else { None }).is_some()
        {
            call_me_msg(event.clone()).await;
            return Ok(());
        }
    }
    //判断消息是否有猫娘两个字
    if event.text.as_ref().and_then(|e| if e.contains("猫娘") { Some(()) } else { None }).is_some()
    {
        method_me(event.clone()).await;
        return Ok(());
    }

    Ok(())
}
async fn call_me_msg(e: Arc<MsgEvent>) {}
async fn method_me(e: Arc<MsgEvent>) {
    e.reply("是不是有人叫我喵");
}
async fn at_me(e: Arc<MsgEvent>) {
    if let Some(cmd) = e.text.as_ref().and_then(|e| if e.starts_with("$") { Some(e) } else { None }) {
        BotCommand::from_str(cmd, e.clone()).invoke_command();
        return;
    }
    e.reply("叫我什么事喵")
}