use human_panic::setup_panic;
use tokio::sync::oneshot;
use tunnelto::{update, Config};

#[tokio::main]
async fn main() {
    let config = match Config::get() {
        Ok(config) => config,
        Err(_) => return,
    };

    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: "support@tunnelto.dev".into(),
        homepage: "https://tunnelto.dev".into(),
    });

    update::check().await;

    let (_txx, rx) = oneshot::channel();
    tunnelto::run(config, None, rx).await
}
