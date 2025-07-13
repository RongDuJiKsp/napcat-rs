mod app;
mod config;

#[kovi::plugin]
async fn main() {
    app::init().await;
}
