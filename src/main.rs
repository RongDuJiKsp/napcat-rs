use kovi::build_bot;

fn main() {
    build_bot!(chat_message, kovi_plugin_check_alllong).run();
}
