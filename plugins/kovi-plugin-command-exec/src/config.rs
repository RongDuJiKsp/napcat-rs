use kovi::MsgEvent;
use kovi_plugin_dev_utils::config;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::OnceLock;

static COMMAND_EXEC_CONFIG: OnceLock<CommandExecConfig> = OnceLock::new();
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CommandExecConfig {
    pub allow_exec_context: HashSet<i64>, //允许执行指令的群组上下文或者私聊上下文
    pub allow_super_user: HashSet<i64>,   //允许执行特命令的用户
    pub is_admin_super_user: bool,        //群管理是否为超级用户
    pub is_all_user_admin: bool,
}
impl CommandExecConfig {
    pub fn event_user(ev: &MsgEvent) -> i64 {
        ev.sender.user_id
    }
    pub fn event_context(ev: &MsgEvent) -> i64 {
        ev.group_id.unwrap_or_else(|| Self::event_user(ev))
    }
    pub fn in_super_user(&self, ev: &MsgEvent) -> bool {
        self.is_all_user_admin
            || self.allow_super_user.contains(&Self::event_user(ev))
            || (self.is_admin_super_user)
    }
    pub fn in_context(&self, ev: &MsgEvent) -> bool {
        self.allow_exec_context.contains(&Self::event_context(ev))
    }
}
config!(CommandExecConfig,COMMAND_EXEC_CONFIG,"command_exec_config.json");