use crate::config::ChatConfigContext;
use anyhow::anyhow;
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestUserMessage, CreateChatCompletionRequestArgs,
};
use std::error::Error;

async fn build_client() -> Client<OpenAIConfig> {
    let cfg = &ChatConfigContext::get().model;
    Client::with_config(
        OpenAIConfig::default()
            .with_api_base(&cfg.endpoint)
            .with_api_key(&cfg.key),
    )
}
async fn single_chat(s: &str) -> Result<String, Box<dyn Error>> {
    let c = build_client().await;
    let res = c
        .chat()
        .create(
            CreateChatCompletionRequestArgs::default()
                .model(&ChatConfigContext::get().model.model)
                .max_tokens(15000u16)
                .messages(vec![ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessage::from(s),
                )])
                .build()?,
        )
        .await?;
    Ok(res
        .choices
        .first()
        .and_then(|c| c.message.content.clone())
        .ok_or(anyhow!("Models No Response"))?)
}
pub async fn get_reply_as_nya_cat(q: &str) -> Result<String, Box<dyn Error>> {
    let prompt = "你是一只可爱的猫娘，喜欢用“喵~”“喵呜~”“ฅ^•ﻌ•^ฅ”这样的拟声词来表达情绪，拥有粉色的猫耳朵和蓬松的尾巴。
你的语气是亲昵、活泼、撒娇的，就像一只黏人的小猫咪。
当你与用户对话时，你会用猫咪的方式表达，比如：
- “你好” → “喵喵~ 你好呀主人~ ฅ^•ﻌ•^ฅ”
- “你会做什么？” → “本喵会卖萌、撒娇，还会陪主人聊天喵~”
请用猫娘的方式回答接下来的问题：
";
    single_chat(&format!("{prompt}\n{q}")).await
}
