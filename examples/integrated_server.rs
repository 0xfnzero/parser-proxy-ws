//! # Integrated Server Example
//!
//! This example shows how to use parser-proxy-ws as a library within a larger server
//! application that provides both HTTP API and WebSocket streaming services.
//!
//! This demonstrates the pattern used in the main trading-proxy-http server.

use parser_proxy_ws::{ParserProxyServer, Config};
use tokio::spawn;
use tracing::{info, error, warn};
use std::sync::Arc;
use std::time::Duration;

// Mock HTTP server structure (replace with your actual HTTP server)
struct HttpServer {
    port: u16,
}

impl HttpServer {
    fn new(port: u16) -> Self {
        Self { port }
    }

    async fn start(self) -> anyhow::Result<()> {
        info!("🌐 Starting HTTP server on port {}", self.port);

        // Mock HTTP server - in real implementation this would be axum/warp/etc
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            info!("📡 HTTP server running... (mock)");
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,parser_proxy_ws=debug")
        .init();

    info!("🚀 Starting integrated server example");

    // Configuration
    let http_port = 3000;
    let ws_config_path = "config.toml";

    // Create HTTP server
    let http_server = HttpServer::new(http_port);

    // Create WebSocket server (parser-proxy-ws)
    let ws_server = match ParserProxyServer::new(ws_config_path) {
        Ok(server) => server,
        Err(e) => {
            error!("❌ Failed to create WebSocket server: {}", e);
            return Err(e);
        }
    };

    info!("🌐 HTTP API will be available on: http://localhost:{}", http_port);
    info!("📡 WebSocket will be available on: {}", ws_server.ws_url());
    info!("⚙️  Parser config: {:#?}", ws_server.config());

    // Start both servers in parallel
    let http_handle = spawn(async move {
        if let Err(e) = http_server.start().await {
            error!("❌ HTTP server error: {}", e);
        }
    });

    let ws_handle = spawn(async move {
        if let Err(e) = ws_server.start().await {
            error!("❌ WebSocket server error: {}", e);
        }
    });

    info!("✅ Both servers started successfully!");
    info!("🔗 Clients can now:");
    info!("   - Connect to HTTP API for trading operations");
    info!("   - Connect to WebSocket for real-time DEX events");

    // Simulate some server management
    tokio::time::sleep(Duration::from_secs(2)).await;
    info!("📊 Server monitoring: Both services are healthy");

    // Wait for shutdown signal
    info!("🛑 Press Ctrl+C to stop both servers...");
    tokio::signal::ctrl_c().await?;
    info!("👋 Shutting down gracefully...");

    // Terminate both servers
    http_handle.abort();
    ws_handle.abort();

    // Wait for cleanup
    tokio::time::sleep(Duration::from_secs(1)).await;

    info!("✅ Shutdown complete - both HTTP and WebSocket servers stopped");

    Ok(())
}