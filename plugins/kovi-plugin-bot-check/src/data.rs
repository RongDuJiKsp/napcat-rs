use kovi::tokio::sync::RwLock;
use kovi_plugin_dev_utils::data;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

static BAN_DATA: OnceLock<Arc<RwLock<BanData>>> = OnceLock::new();
#[derive(Serialize, Deserialize, Default)]
pub struct BanData {
    pub chat_action_times: HashMap<i64, HashMap<i64, i32>>, //每个群内每个人触发的次数
    pub invite_action_times: HashMap<i64, HashMap<i64, i32>>, //每个群内每个人触发的次数
}
data!(BanData, BAN_DATA, "ban_data.json");
