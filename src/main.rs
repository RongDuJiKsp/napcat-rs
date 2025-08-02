use kovi::build_bot;

fn main() {
    build_bot!(
        kovi_plugin_command_exec,
        kovi_plugin_javascript_shell,
        kovi_plugin_check_alllong,
        chat_message,
        kovi_plugin_emoji_attack,
    )
        .run();
}
