use crate::config::CommandExecConfig;
use kovi::PluginBuilder as plugin;
pub mod app;
pub mod config;
#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    CommandExecConfig::init(&bot).unwrap();
}
