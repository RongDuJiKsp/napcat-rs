use crate::shell::register_shell_cmd;
use kovi::PluginBuilder as plugin;

pub mod shell;

#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    register_shell_cmd(bot).await;
}
