mod app;
mod config;
mod handle;
mod data;

#[kovi::plugin]
async fn main() {
    app::init().await;
}
