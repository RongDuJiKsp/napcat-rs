use kovi_plugin_dev_utils::config;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::VecSkipError;
use serde_with::{serde_as, DisplayFromStr};
use std::sync::OnceLock;

static BAN_CONFIG: OnceLock<BanConfig> = OnceLock::new();
#[serde_as]
#[derive(Default, Deserialize, Serialize)]
pub struct BanConfig {
    enable_group: Vec<i64>, //启用上下文群组
    #[serde_as(as = "VecSkipError<DisplayFromStr>")]
    chat_regex_list: Vec<Regex>, //触发发言匹配的正则表达式列表
    chat_enable_shut_up: Option<i32>, //触发达到次数自动禁言
    enable_chat_kick: Option<i32>, //触发发言ban达到次数时自动ban
    enable_invite_ban: Option<InviteBanConfig>, //群内邀请ban处理配置
    enable_invite_kick: Option<i32>, //触发邀请ban达到次数时自动ban
}
config!(BanConfig, BAN_CONFIG, "ban_config.json");
#[derive(Default, Deserialize, Serialize)]
pub struct InviteBanConfig {
    min_level: Option<i32>,    //当邀请人等级小于这个数时触发ban
    min_activate: Option<i32>, //当邀请人群活跃等级小于这个数时
}
