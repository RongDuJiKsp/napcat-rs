use crate::shell::register_shell_cmd;
use kovi::PluginBuilder as plugin;
use kovi_plugin_command_exec::config::CommandExecConfig;

pub mod shell;

#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    CommandExecConfig::init(&bot).await.unwrap();
    register_shell_cmd(bot).await;
}
