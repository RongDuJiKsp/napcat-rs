use kovi::tokio::sync::RwLock;
use kovi::RuntimeBot;
use kovi_plugin_dev_utils::config::initfn::init_data;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};

static EMOJI_ATTACK_DATA: OnceLock<Arc<RwLock<EmojiAttackData>>> = OnceLock::new();
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct EmojiAttackData {
    pub group_users: HashMap<i64, HashSet<i64>>,
}
impl EmojiAttackData {
    pub fn init(runtime_bot: &RuntimeBot) -> Result<(), anyhow::Error> {
        init_data(runtime_bot, "emoji_attack_data.json", &EMOJI_ATTACK_DATA)
    }
    pub fn get() -> Arc<RwLock<Self>> {
        EMOJI_ATTACK_DATA.get().unwrap().clone()
    }
}
