use anyhow::anyhow;
use kovi::utils::load_json_data;
use kovi::RuntimeBot;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

static CHAT_CONFIG: OnceLock<ChatConfigContext> = OnceLock::new();
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ChatModelCallConfig {
    //openai configs
    pub key: String,
    pub endpoint: String,
    pub max_tokens: u16,
    //角色扮演机器人相关
    pub role_model: String,
    pub role_prompt: String,
    pub role_context_expiration_time_second: u64,//角色扮演机器人的对话记忆过期时间
    pub role_max_message: usize,//角色扮演机器人的对话窗口大小
    //聪明机器人相关
    pub smart_model: String,
    pub smart_prompt: String,
    //机器人对话拆分相关
    pub dot_wait_tag: String,//机器人大段话变成对话的分隔符
    pub dot_wait_time_ms: u64,//机器人发这大段话的时间
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
    pub async fn init(runtime_bot: &RuntimeBot) -> Result<(), anyhow::Error> {
        let config = load_json_data(
            ChatConfig::default(),
            runtime_bot.get_data_path().join("chat_config.json"),
        )
        .map_err(|e| anyhow!("Error loading chat config: {}", e))?;
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
pub struct SyncControl;
static LIVE: AtomicBool = AtomicBool::new(true);
impl SyncControl {
    pub fn set_bot_run(run: bool) {
        LIVE.store(run, Ordering::Relaxed);
    }
    pub fn running() -> bool {
        LIVE.load(Ordering::Relaxed)
    }
}
