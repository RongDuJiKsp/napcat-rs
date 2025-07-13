use kovi::RuntimeBot;
use kovi_plugin_dev_utils::configinit::init_config;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::OnceLock;

static EMOJI_ATTACK_CONFIG: OnceLock<EmojiAttackConfig> = OnceLock::new();
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmojiAttackConfig {
    pub allow_monkey_groups: HashSet<i64>, //允许对标记的用户贴emoji的群组上下文
    pub emoji: String,
}

impl EmojiAttackConfig {
    pub fn init(runtime_bot: &RuntimeBot) -> Result<(), anyhow::Error> {
        init_config(
            runtime_bot,
            "emoji_attack_config.json",
            &EMOJI_ATTACK_CONFIG,
        )
    }
    pub fn get() -> &'static EmojiAttackConfig {
        EMOJI_ATTACK_CONFIG.get().unwrap()
    }
}
