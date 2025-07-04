use crate::command_exec::app::BotCommand;
use crate::config::{ChatConfigContext, SyncControl};
use crate::tools::find_group;
use crate::{ml, tools};
use anyhow::anyhow;
use async_openai::types::{
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
};
use kovi::log::{error, info};
use kovi::tokio::sync::RwLock;
use kovi::{MsgEvent, RuntimeBot};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, OnceLock};
use std::time::SystemTime;

pub async fn handle_group_chat(
    bot: Arc<RuntimeBot>,
    event: Arc<MsgEvent>,
) -> Result<(), anyhow::Error> {
    //只考虑已经监听的群
    if !ChatConfigContext::get()
        .allow_groups
        .contains(&event.group_id.ok_or(anyhow!("找不到群id"))?)
    {
        return Ok(());
    }
    //有人@猫娘
    if event.message.contains("at")
        && event
            .message
            .get("at")
            .get(0)
            .and_then(|s| s.data.get("qq"))
            .and_then(|v| v.as_str().and_then(|s| s.parse::<i64>().ok()))
            .and_then(|e| if e == event.self_id { Some(()) } else { None })
            .is_some()
    {
        at_me(event.clone()).await;
        return Ok(());
    }

    for g in find_group(&event) {
        info!("读取到可能为群号的号码{}，正在尝试加群", g);
        // 添加请求
    }
    let bot_info = tools::self_bot_info(&bot, &event).await.ok();
    //若有bot info
    if let Some(bot_if) = &bot_info {
        //判断消息是否有猫娘的名字
        if event
            .text
            .as_ref()
            .and_then(|e| {
                if e.contains(&bot_if.nickname) {
                    Some(())
                } else {
                    None
                }
            })
            .is_some()
        {
            call_me_msg(event.clone()).await;
            return Ok(());
        }
    }
    //判断消息是否有猫娘两个字
    if event
        .text
        .as_ref()
        .and_then(|e| if e.contains("猫娘") { Some(()) } else { None })
        .is_some()
    {
        method_me(event.clone()).await;
        return Ok(());
    }

    Ok(())
}
async fn call_me_msg(e: Arc<MsgEvent>) {
    at_me(e).await;
}

async fn method_me(e: Arc<MsgEvent>) {
    if !SyncControl::running() {
        return;
    }
    e.reply("是不是有人叫我喵");
}
type UnixTime = u64;
#[derive(Debug, Default)]
pub struct NyaCatMemory {
    //user chat time and message
    user_memory: HashMap<i64, VecDeque<(UnixTime, ChatCompletionRequestMessage)>>,
}
static CAT_MEMORY: OnceLock<RwLock<NyaCatMemory>> = OnceLock::new();
impl NyaCatMemory {
    pub fn load() -> &'static RwLock<NyaCatMemory> {
        CAT_MEMORY.get_or_init(|| RwLock::new(NyaCatMemory::default()))
    }
    fn system_msg() -> ChatCompletionRequestMessage {
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage::from(
            ChatConfigContext::get().model.role_model.as_str(),
        ))
    }
    pub fn clean(&mut self) {
        self.user_memory.clear();
    }
    fn load_mem(&mut self, user_id: i64, new_msg: &str) -> Vec<ChatCompletionRequestMessage> {
        info!("群聊或用户{user_id}发出提问:{new_msg}");
        let now_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System Time Error!!!!!!")
            .as_secs();
        let arr = self.user_memory.entry(user_id).or_default();
        arr.push_back((
            now_time,
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage::from(new_msg)),
        ));
        while let Some((chat_time, msg)) = arr.pop_front() {
            if arr.len() < ChatConfigContext::get().model.role_max_message
                && now_time - chat_time
                    < ChatConfigContext::get()
                        .model
                        .role_context_expiration_time_second
            {
                arr.push_front((chat_time, msg));
                break;
            }
        }
        let mut v = vec![Self::system_msg()];
        v.append(&mut arr.iter().cloned().map(|x| x.1).collect());
        v
    }
    fn save_mem(&mut self, user_id: i64, new_chat_msg: &str) {
        info!("模型对群聊或用户{user_id}回答:{new_chat_msg}");
        let now_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System Time Error!!!!!!")
            .as_secs();
        self.user_memory.entry(user_id).or_default().push_back((
            now_time,
            ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage::from(
                new_chat_msg,
            )),
        ))
    }
}
async fn at_me(e: Arc<MsgEvent>) {
    //如果是指令则处理指令
    if let Some(cmd) = e
        .text
        .as_ref()
        .and_then(|e| if e.starts_with("$") { Some(e) } else { None })
    {
        BotCommand::from_str(cmd, e.clone()).invoke_command().await;
        return;
    }

    //否则当成问话
    if !SyncControl::running() {
        //如果关闭了则不响应问话
        return;
    }
    if let Some(question) = e
        .text
        .as_ref()
        .and_then(|s| if s.len() > 0 { Some(s) } else { None })
    {
        let chat = NyaCatMemory::load()
            .write()
            .await
            .load_mem(e.group_id.unwrap_or(e.sender.user_id), question);
        match ml::get_reply_as_nya_cat(chat).await {
            Ok(out) => {
                NyaCatMemory::load()
                    .write()
                    .await
                    .save_mem(e.sender.user_id, &out);
                e.reply_and_quote(out);
            }
            Err(err) => {
                e.reply_and_quote(format!("发生错误了喵：{}", err));
                error!("模型在回复时发生错误：{}", err);
            }
        }
    } else {
        e.reply_and_quote("叫我什么事喵？");
    }
}
