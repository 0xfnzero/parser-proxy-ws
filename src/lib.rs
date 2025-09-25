//! # Parser Proxy WebSocket Library
//!
//! A high-performance Solana DEX event parsing and WebSocket streaming library.
//!
//! ## Features
//!
//! - **Ultra-fast parsing**: 10-20μs latency for real-time trading applications
//! - **Multi-DEX support**: PumpFun, PumpSwap, Raydium, Orca, Meteora, and more
//! - **WebSocket streaming**: Real-time event broadcasting to multiple clients
//! - **Configurable filtering**: Fine-grained control over which events to monitor
//! - **Easy integration**: Simple API with just a config file path
//!
//! ## Usage
//!
//! ```rust
//! use parser_proxy_ws::ParserProxyServer;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Start the parser proxy server with a config file
//!     let server = ParserProxyServer::new("config.toml")?;
//!     server.start().await?;
//!     Ok(())
//! }
//! ```

use sol_parser_sdk::grpc::{
    AccountFilter, ClientConfig, EventTypeFilter, TransactionFilter,
    YellowstoneGrpc,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info, warn};
use anyhow::Result;

mod config;
mod ws_server;

pub use config::Config;
use ws_server::WsServer;

/// The main parser proxy server that handles gRPC subscriptions and WebSocket broadcasting
pub struct ParserProxyServer {
    config: Config,
}

impl ParserProxyServer {
    /// Create a new parser proxy server with the specified config file path
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the TOML configuration file
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parser_proxy_ws::ParserProxyServer;
    ///
    /// let server = ParserProxyServer::new("config.toml")?;
    /// ```
    pub fn new<P: AsRef<std::path::Path>>(config_path: P) -> Result<Self> {
        let config = Config::load_or_default(config_path.as_ref().to_str().unwrap());

        Ok(Self { config })
    }

    /// Create a new parser proxy server with an existing config
    ///
    /// # Arguments
    ///
    /// * `config` - Pre-loaded configuration
    pub fn with_config(config: Config) -> Self {
        Self { config }
    }

    /// Start the parser proxy server
    ///
    /// This method will:
    /// 1. Initialize the WebSocket server
    /// 2. Connect to the Yellowstone gRPC endpoint
    /// 3. Subscribe to configured DEX events
    /// 4. Start broadcasting events to WebSocket clients
    ///
    /// The method will run indefinitely until interrupted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use parser_proxy_ws::ParserProxyServer;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let server = ParserProxyServer::new("config.toml")?;
    ///     server.start().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn start(self) -> Result<()> {
        self.init_tracing();

        info!("🚀 Starting Parser Proxy WebSocket Server...");

        let ws_server = Arc::new(WsServer::new());
        let ws_server_clone = ws_server.clone();

        // Start WebSocket server
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        let listener = TcpListener::bind(&addr).await?;
        info!("📡 WebSocket server listening on: ws://{}", addr);

        tokio::spawn(async move {
            ws_server_clone.run(listener).await;
        });

        // Start gRPC client and event processing
        self.start_grpc_processing(ws_server).await?;

        Ok(())
    }

    /// Start the parser proxy server and wait for shutdown signal
    ///
    /// This is a convenience method that calls `start()` and waits for Ctrl+C.
    pub async fn run(self) -> Result<()> {
        let server_task = tokio::spawn(async move {
            if let Err(e) = self.start().await {
                error!("Server error: {}", e);
            }
        });

        info!("🛑 Press Ctrl+C to stop...");
        tokio::signal::ctrl_c().await?;
        info!("👋 Shutting down gracefully...");

        server_task.abort();
        Ok(())
    }

    /// Get the server configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the WebSocket URL that clients can connect to
    pub fn ws_url(&self) -> String {
        format!("ws://{}:{}", self.config.server.host, self.config.server.port)
    }

    fn init_tracing(&self) {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "info");
        }

        #[cfg(feature = "binary")]
        {
            tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
                )
                .init();
        }

        let _ = rustls::crypto::ring::default_provider().install_default();
    }

    async fn start_grpc_processing(
        &self,
        ws_server: Arc<WsServer>,
    ) -> Result<()> {
        let mut grpc_config = ClientConfig::default();
        grpc_config.enable_metrics = self.config.grpc.enable_metrics;
        grpc_config.connection_timeout_ms = self.config.grpc.connection_timeout_ms;
        grpc_config.request_timeout_ms = self.config.grpc.request_timeout_ms;
        grpc_config.enable_tls = self.config.grpc.enable_tls;

        let grpc = YellowstoneGrpc::new_with_config(
            self.config.grpc.endpoint.clone(),
            self.config.grpc.token.clone(),
            grpc_config,
        )
        .map_err(|e| anyhow::anyhow!("gRPC client creation failed: {}", e))?;

        info!("✅ gRPC client created successfully");

        let protocols = self.config.get_enabled_protocols();
        if protocols.is_empty() {
            warn!("⚠️  No protocols enabled in config, server will receive no events");
            return Ok(());
        }

        info!("📊 Monitoring protocols: {:?}", protocols);

        let transaction_filter = TransactionFilter::for_protocols(&protocols);
        let account_filter = AccountFilter::for_protocols(&protocols);

        let event_types = self.config.get_enabled_event_types();
        if event_types.is_empty() {
            warn!("⚠️  No event types enabled in config, server will receive no events");
            return Ok(());
        }

        info!("🎯 Monitoring event types: {:?}", event_types);
        let event_filter = EventTypeFilter::include_only(event_types);

        info!("🎧 Starting subscription...");

        let queue = grpc
            .subscribe_dex_events(
                vec![transaction_filter],
                vec![account_filter],
                Some(event_filter),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Subscription failed: {}", e))?;

        info!("✅ Subscription established");

        tokio::spawn(async move {
            let mut spin_count = 0u32;
            loop {
                if let Some(event) = queue.pop() {
                    spin_count = 0;

                    let event_json = match serde_json::to_string(&event) {
                        Ok(json) => json,
                        Err(e) => {
                            error!("Failed to serialize event: {}", e);
                            continue;
                        }
                    };

                    ws_server.broadcast(&event_json).await;
                } else {
                    spin_count += 1;
                    if spin_count < 1000 {
                        std::hint::spin_loop();
                    } else {
                        tokio::task::yield_now().await;
                        spin_count = 0;
                    }
                }
            }
        });

        // Keep the main task alive
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

/// Convenience function to start a parser proxy server with default settings
///
/// # Arguments
///
/// * `config_path` - Path to the TOML configuration file
///
/// # Examples
///
/// ```rust
/// use parser_proxy_ws::run_server;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     run_server("config.toml").await?;
///     Ok(())
/// }
/// ```
pub async fn run_server<P: AsRef<std::path::Path>>(config_path: P) -> Result<()> {
    let server = ParserProxyServer::new(config_path)?;
    server.run().await
}

/// Re-export commonly used types for convenience
pub mod prelude {
    pub use crate::{ParserProxyServer, Config, run_server};
    pub use sol_parser_sdk::core::events::*;
    pub use sol_parser_sdk::grpc::EventType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_server_creation() {
        // Create a minimal test config
        let test_config = r#"
[server]
host = "127.0.0.1"
port = 9001

[grpc]
endpoint = "https://test-endpoint.com"
token = ""
enable_metrics = false
enable_tls = true
connection_timeout_ms = 5000
request_timeout_ms = 10000

[protocols]
pumpfun = true

[events]
pumpfun_trade = true
"#;

        fs::write("test_config.toml", test_config).unwrap();

        let server = ParserProxyServer::new("test_config.toml");
        assert!(server.is_ok());

        let server = server.unwrap();
        assert_eq!(server.config().server.host, "127.0.0.1");
        assert_eq!(server.config().server.port, 9001);
        assert_eq!(server.ws_url(), "ws://127.0.0.1:9001");

        // Cleanup
        fs::remove_file("test_config.toml").unwrap();
    }
}