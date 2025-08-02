mod dynconfig;

use kovi::PluginBuilder as plugin;
use crate::dynconfig::BanConfig;

#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    BanConfig::init(&bot).expect("Failed to initialize BanDynConfig");
}
