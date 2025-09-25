//! # Library Integration Example
//!
//! This example shows how to use parser-proxy-ws as a library component
//! within your own application, with custom configuration and error handling.
//!
//! Perfect for integrating into existing Rust projects that need DEX event streaming.

use parser_proxy_ws::{ParserProxyServer, Config};
use tracing::{info, error, warn, debug};
use tokio::sync::mpsc;
use std::time::Duration;

struct TradingApplication {
    ws_server: Option<ParserProxyServer>,
    event_receiver: mpsc::UnboundedReceiver<String>,
}

impl TradingApplication {
    fn new() -> anyhow::Result<Self> {
        let (_tx, rx) = mpsc::unbounded_channel();

        Ok(Self {
            ws_server: None,
            event_receiver: rx,
        })
    }

    async fn initialize_parser_proxy(&mut self, config_path: &str) -> anyhow::Result<()> {
        info!("🔧 Initializing parser proxy with config: {}", config_path);

        // Create the server
        let server = ParserProxyServer::new(config_path)?;

        // Log server info
        info!("📡 WebSocket will be available at: {}", server.ws_url());
        debug!("⚙️  Server configuration loaded successfully");

        // Check config for enabled protocols
        let config = server.config();
        if config.get_enabled_protocols().is_empty() {
            warn!("⚠️  No protocols enabled - server will receive no events");
        } else {
            info!("📊 Monitoring protocols: {:?}", config.get_enabled_protocols());
        }

        self.ws_server = Some(server);
        Ok(())
    }

    async fn start_services(&mut self) -> anyhow::Result<()> {
        info!("🚀 Starting all services...");

        // Start the parser proxy WebSocket server
        if let Some(server) = self.ws_server.take() {
            let ws_url = server.ws_url();

            tokio::spawn(async move {
                info!("📡 Starting WebSocket server...");
                if let Err(e) = server.start().await {
                    error!("❌ WebSocket server failed: {}", e);
                }
            });

            info!("✅ WebSocket server started at: {}", ws_url);
        }

        // Start other application services
        self.start_trading_logic().await?;

        Ok(())
    }

    async fn start_trading_logic(&self) -> anyhow::Result<()> {
        info!("🤖 Starting trading logic...");

        // Simulate trading application logic
        tokio::spawn(async {
            loop {
                tokio::time::sleep(Duration::from_secs(10)).await;
                info!("📈 Processing trading signals... (mock)");

                // In a real application, you would:
                // 1. Connect to the WebSocket endpoint as a client
                // 2. Parse incoming DEX events
                // 3. Execute your trading strategy
                // 4. Make buy/sell decisions
                // 5. Execute trades via your preferred method
            }
        });

        Ok(())
    }

    async fn run(&mut self) -> anyhow::Result<()> {
        info!("🎯 Running trading application");

        // Main application loop
        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("👋 Shutdown signal received");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(30)) => {
                    info!("💓 Application health check: OK");
                }
                // In a real app, you might have other select branches for:
                // - Processing events from event_receiver
                // - Handling API requests
                // - Database operations
                // - etc.
            }
        }

        info!("✅ Application shutdown complete");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter("info,parser_proxy_ws=debug")
        .init();

    info!("🚀 Starting trading application with parser-proxy-ws library");

    // Create and configure the application
    let mut app = TradingApplication::new()?;

    // Initialize the parser proxy component
    let config_path = std::env::var("PARSER_CONFIG_PATH")
        .unwrap_or_else(|_| "config.toml".to_string());

    if let Err(e) = app.initialize_parser_proxy(&config_path).await {
        error!("❌ Failed to initialize parser proxy: {}", e);
        error!("💡 Make sure {} exists and contains valid configuration", config_path);
        return Err(e);
    }

    // Start all services
    app.start_services().await?;

    info!("🎉 All services started successfully!");
    info!("📋 Application features:");
    info!("   ✅ Real-time DEX event streaming via WebSocket");
    info!("   ✅ Trading logic processing");
    info!("   ✅ Health monitoring");
    info!("   ✅ Graceful shutdown handling");

    // Run the main application loop
    app.run().await?;

    Ok(())
}