mod dynconfig;

use kovi::PluginBuilder as plugin;
use crate::dynconfig::BanDynConfig;

#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    BanDynConfig::init(&bot).expect("Failed to initialize BanDynConfig");
}
