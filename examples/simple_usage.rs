//! # Simple Usage Example
//!
//! This example shows how to use the parser-proxy-ws library in its simplest form.
//! Just provide a config file path and the library handles everything else.

use parser_proxy_ws::ParserProxyServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start the parser proxy server with a config file
    // This will run indefinitely, streaming events via WebSocket
    let server = ParserProxyServer::new("config.toml")?;
    server.start().await?;

    Ok(())
}