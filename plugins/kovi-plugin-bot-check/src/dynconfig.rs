use kovi_plugin_dev_utils::config;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static BAN_DYN_CONFIG: OnceLock<BanDynConfig> = OnceLock::new();
#[derive(Default, Deserialize, Serialize)]
pub struct BanDynConfig {}
config!(BanDynConfig, BAN_DYN_CONFIG,"ban_dyn_config.json");