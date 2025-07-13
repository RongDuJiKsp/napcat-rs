use anyhow::anyhow;
use kovi::utils::load_json_data;
use kovi::RuntimeBot;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::OnceLock;

static EMOJI_ATTACK_CONFIG: OnceLock<EmojiAttackContext> = OnceLock::new();
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmojiAttackConfig {
    pub allow_monkey_groups: Vec<i64>, //允许对标记的用户贴emoji的群组上下文
    pub emoji: String,
}
#[derive(Clone, Debug)]
pub struct EmojiAttackContext {
    pub allow_monkey_groups: HashSet<i64>,
    pub emoji: String,
}
impl EmojiAttackContext {
    fn from_config(cfg: &EmojiAttackConfig) -> EmojiAttackContext {
        EmojiAttackContext {
            allow_monkey_groups: cfg.allow_monkey_groups.iter().copied().collect(),
            emoji: cfg.emoji.clone(),
        }
    }
}
impl EmojiAttackConfig {
    pub async fn init(runtime_bot: &RuntimeBot) -> Result<(), anyhow::Error> {
        let config = load_json_data(
            EmojiAttackConfig::default(),
            runtime_bot.get_data_path().join("emoji_attack_config.json"),
        )
        .map_err(|e| anyhow!("Error loading command config: {}", e))?;
        EMOJI_ATTACK_CONFIG
            .set(EmojiAttackContext::from_config(&config))
            .map_err(|_e| anyhow!("初始化CommandConfigContext时出现重复设置"))?;
        Ok(())
    }
    pub fn get() -> &'static EmojiAttackContext {
        EMOJI_ATTACK_CONFIG.get().unwrap()
    }
}
