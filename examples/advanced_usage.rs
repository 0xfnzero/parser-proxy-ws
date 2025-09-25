//! # Advanced Usage Example
//!
//! This example shows how to use the ParserProxyServer directly for more control
//! over configuration, error handling, and shutdown behavior.

use parser_proxy_ws::{ParserProxyServer, Config};
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize custom logging
    tracing_subscriber::fmt()
        .with_env_filter("info,parser_proxy_ws=debug")
        .init();

    info!("🚀 Starting advanced parser proxy example");

    // Create server with config file path
    let server = match ParserProxyServer::new("config.toml") {
        Ok(server) => server,
        Err(e) => {
            error!("Failed to create server: {}", e);
            return Err(e);
        }
    };

    // Get server info
    info!("📡 WebSocket URL: {}", server.ws_url());
    info!("⚙️  Server config: {:#?}", server.config());

    // Start the server with custom shutdown handling
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            error!("Server error: {}", e);
        }
    });

    // Wait for shutdown signal
    info!("🛑 Press Ctrl+C to stop...");
    tokio::signal::ctrl_c().await?;
    info!("👋 Shutting down gracefully...");

    // Clean shutdown
    server_handle.abort();

    // Wait a bit for cleanup
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    info!("✅ Shutdown complete");

    Ok(())
}