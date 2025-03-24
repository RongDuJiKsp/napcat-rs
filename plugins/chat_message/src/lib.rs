mod handlers;
mod config;
mod tools;

use crate::config::ChatConfigContext;
use kovi::log::error;
use kovi::PluginBuilder as plugin;

#[kovi::plugin]
async fn main() {
    app().await;
}
async fn app() {
    let bot = plugin::get_runtime_bot();
    ChatConfigContext::init(&bot).await.expect("error on load ChatConfigContext");
    plugin::on_group_msg(|event| async {
        if let Err(error) = handlers::group_chat::handle_group_chat(event).await {
            error!("{error}")
        }
    })
}