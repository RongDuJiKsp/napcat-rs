use anyhow::anyhow;
use kovi::utils::load_json_data;
use kovi::RuntimeBot;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::OnceLock;

static COMMAND_EXEC_CONFIG: OnceLock<CommandExecContext> = OnceLock::new();
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CommandExecConfig {
    pub allow_exec_groups: Vec<i64>, //允许执行指令的群组上下文或者私聊上下文
    pub allow_super_user: Vec<i64>,  //允许执行特命令的用户
}
#[derive(Clone, Debug)]
pub struct CommandExecContext {
    pub allow_exec_groups: HashSet<i64>,
    pub allow_super_user: HashSet<i64>,
}
impl CommandExecContext {
    fn from_config(cfg: &CommandExecConfig) -> CommandExecContext {
        CommandExecContext {
            allow_exec_groups: cfg.allow_exec_groups.iter().copied().collect(),
            allow_super_user: cfg.allow_super_user.iter().copied().collect(),
        }
    }
}
impl CommandExecConfig {
    pub async fn init(runtime_bot: &RuntimeBot) -> Result<(), anyhow::Error> {
        let config = load_json_data(
            CommandExecConfig::default(),
            runtime_bot.get_data_path().join("chat_config.json"),
        )
        .map_err(|e| anyhow!("Error loading chat config: {}", e))?;
        COMMAND_EXEC_CONFIG
            .set(CommandExecContext::from_config(&config))
            .map_err(|_e| anyhow!("初始化ChatConfigContext时出现重复设置"))?;
        Ok(())
    }
    pub fn get() -> &'static CommandExecContext {
        COMMAND_EXEC_CONFIG.get().unwrap()
    }
}
