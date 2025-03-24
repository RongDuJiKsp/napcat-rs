use anyhow::anyhow;
use kovi::utils::load_json_data;
use kovi::RuntimeBot;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::sync::OnceLock;

static CHAT_CONFIG: OnceLock<ChatConfigContext> = OnceLock::new();
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatConfig {
    allow_groups: Vec<i64>,
}
#[derive(Debug)]
pub struct ChatConfigContext {
    pub allow_groups: HashSet<i64>,
}
impl ChatConfigContext {
    pub async fn init(runtime_bot: &RuntimeBot) -> Result<(), Box<dyn Error>> {
        let default_config: ChatConfig = ChatConfig {
            allow_groups: vec![]
        };
        let config = load_json_data(default_config, runtime_bot.get_data_path().join("chat_config.json"))?;
        CHAT_CONFIG.set(ChatConfigContext::from_config(&config)).map_err(|_e| anyhow!("初始化ChatConfigContext时出现重复设置"))?;
        Ok(())
    }
    pub fn get() -> &'static ChatConfigContext {
        CHAT_CONFIG.get().unwrap()
    }
    pub fn from_config(value: &ChatConfig) -> ChatConfigContext {
        ChatConfigContext {
            allow_groups: value.allow_groups.iter().copied().collect()
        }
    }
}