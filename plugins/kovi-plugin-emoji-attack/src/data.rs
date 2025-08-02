use kovi::tokio::sync::RwLock;
use kovi_plugin_dev_utils::data;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};

static EMOJI_ATTACK_DATA: OnceLock<Arc<RwLock<EmojiAttackData>>> = OnceLock::new();
#[derive(Serialize, Default, Deserialize, Debug)]
pub struct EmojiAttackData {
    pub group_users: HashMap<i64, HashSet<i64>>,
}
data!(EmojiAttackData, EMOJI_ATTACK_DATA, "emoji_attack_data.json");
