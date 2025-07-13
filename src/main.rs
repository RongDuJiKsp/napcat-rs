use kovi::build_bot;

fn main() {
    build_bot!(
        kovi_plugin_command_exec,
        kovi_plugin_javascript_shell,
        chat_message,
    )
    .run();
}
