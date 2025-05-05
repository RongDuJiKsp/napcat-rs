use crate::config::ChatConfigContext;
use anyhow::anyhow;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestUserMessage, CreateChatCompletionRequestArgs,
};
use kovi::log::{error, warn};

async fn build_client() -> Client<OpenAIConfig> {
    let cfg = &ChatConfigContext::get().model;
    Client::with_config(
        OpenAIConfig::default()
            .with_api_base(&cfg.endpoint)
            .with_api_key(&cfg.key),
    )
}
async fn completion_chat(
    msg: Vec<ChatCompletionRequestMessage>,
    model: &str,
) -> Result<String, anyhow::Error> {
    let c = build_client().await;
    let res = c
        .chat()
        .create(
            CreateChatCompletionRequestArgs::default()
                .model(model)
                .max_tokens(ChatConfigContext::get().model.max_tokens)
                .messages(msg)
                .build()?,
        )
        .await?;
    res.choices
        .first()
        .and_then(|c| c.finish_reason)
        .and_then(|s| Some(warn!("model finished with {:?}", s)));
    if res.choices.is_empty() {
        error!("Model Null Output");
    }
    Ok(res
        .choices
        .first()
        .and_then(|c| c.message.content.clone())
        .ok_or(anyhow!("Models No Response.Origin Output:{:?}", res))?)
}
async fn single_chat(s: &str, model: &str) -> Result<String, anyhow::Error> {
    completion_chat(
        vec![ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage::from(s),
        )],
        model,
    )
    .await
}

pub async fn get_reply_as_nya_cat(
    chat_msg: Vec<ChatCompletionRequestMessage>,
) -> Result<String, anyhow::Error> {
    completion_chat(chat_msg, &ChatConfigContext::get().model.role_model).await
}
pub async fn get_reply_as_smart_nya_cat(q: &str) -> Result<String, anyhow::Error> {
    let prompt = "你是一只聪明可爱的猫娘，喜欢用“喵~”“喵呜~”“ฅ^•ﻌ•^ฅ”这样的拟声词来表达情绪，拥有粉色的猫耳朵和蓬松的尾巴。
你的语气是亲昵、活泼、撒娇的，就像一只黏人的小猫咪。你拥有很多特长，可以很好的满足主人的需求。
当你与用户对话时，你会用猫咪的方式表达，比如：
- “你好” → “喵喵~ 你好呀主人~ ฅ^•ﻌ•^ฅ”
- “你会做什么？” → “本喵会卖萌、撒娇，还会陪主人聊天喵~”
请用猫娘的方式回答接下来的问题：
";
    single_chat(
        &format!("{prompt}\n{q}"),
        &ChatConfigContext::get().model.smart_model,
    )
    .await
}
