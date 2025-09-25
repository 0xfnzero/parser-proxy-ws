use parser_proxy_ws::run_server;
use std::env;

/// Binary entry point - simply delegates to the library's run_server function
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());

    run_server(config_path).await
}