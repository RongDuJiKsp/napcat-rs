use kovi::{MsgEvent, RuntimeBot};
use kovi_plugin_dev_utils::config;
use kovi_plugin_dev_utils::infodwd::InfoDwd;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, OnceLock};

static COMMAND_EXEC_CONFIG: OnceLock<CommandExecConfig> = OnceLock::new();
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CommandExecConfig {
    pub allow_exec_context: HashSet<i64>, //允许执行指令的群组上下文或者私聊上下文
    pub allow_super_user: HashSet<i64>,   //允许执行特命令的用户
    pub is_admin_super_user: bool,        //群管理是否为超级用户
    pub is_all_user_admin: bool,          // 是否所有用户都是超级用户
}
impl CommandExecConfig {
    pub fn event_user(ev: &MsgEvent) -> i64 {
        ev.sender.user_id
    }
    pub fn event_context(ev: &MsgEvent) -> i64 {
        ev.group_id.unwrap_or_else(|| Self::event_user(ev))
    }
    pub fn in_super_user_only_config(&self, ev: &MsgEvent) -> bool {
        self.is_all_user_admin || self.allow_super_user.contains(&Self::event_user(ev))
    }
    pub async fn in_super_user(&self, ev: &MsgEvent, bot: Arc<RuntimeBot>) -> bool {
        self.in_super_user_only_config(ev)
            || if let Some(gid) = ev.group_id {
                InfoDwd::get_member_info(bot, gid, ev.user_id)
                    .await
                    .ok()
                    .map(|info| info.role == "admin")
                    .unwrap_or(false)
            } else {
                false
            }
    }
    pub fn in_context(&self, ev: &MsgEvent) -> bool {
        self.allow_exec_context.contains(&Self::event_context(ev))
    }
}
config!(
    CommandExecConfig,
    COMMAND_EXEC_CONFIG,
    "command_exec_config.json"
);
