use anyhow::anyhow;
use kovi::RuntimeBot;
use kovi::utils::load_json_data;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::sync::OnceLock;

static CHAT_CONFIG: OnceLock<ChatConfigContext> = OnceLock::new();
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChatModelCallConfig {
    pub key: String,
    pub endpoint: String,
    pub max_tokens: u16,
    pub role_model: String,
    pub role_context_expiration_time_second: u64,
    pub role_max_message: usize,
    pub smart_model: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChatConfig {
    allow_groups: Vec<i64>,
    allow_super_user: Vec<i64>,
    model: ChatModelCallConfig,
}
#[derive(Debug)]
pub struct ChatConfigContext {
    pub allow_groups: HashSet<i64>,
    pub allow_super_user: HashSet<i64>,
    pub model: ChatModelCallConfig,
}
impl ChatConfigContext {
    pub async fn init(runtime_bot: &RuntimeBot) -> Result<(), Box<dyn Error>> {
        let config = load_json_data(
            ChatConfig::default(),
            runtime_bot.get_data_path().join("chat_config.json"),
        )?;
        CHAT_CONFIG
            .set(ChatConfigContext::from_config(&config))
            .map_err(|_e| anyhow!("初始化ChatConfigContext时出现重复设置"))?;
        Ok(())
    }
    pub fn get() -> &'static ChatConfigContext {
        CHAT_CONFIG.get().unwrap()
    }
    pub fn from_config(value: &ChatConfig) -> ChatConfigContext {
        ChatConfigContext {
            allow_groups: value.allow_groups.iter().copied().collect(),
            allow_super_user: value.allow_super_user.iter().copied().collect(),
            model: value.model.clone(),
        }
    }
}
