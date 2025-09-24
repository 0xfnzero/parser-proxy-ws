use sol_parser_sdk::grpc::{
    AccountFilter, ClientConfig, EventTypeFilter, TransactionFilter,
    YellowstoneGrpc,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};

mod config;
mod ws_server;

use config::Config;
use ws_server::WsServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let _ = rustls::crypto::ring::default_provider().install_default();

    info!("🚀 Starting Parser Proxy WebSocket Server...");

    let config = Config::load_or_default("config.toml");

    let ws_server = Arc::new(WsServer::new());
    let ws_server_clone = ws_server.clone();

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("📡 WebSocket server listening on: ws://{}", addr);

    tokio::spawn(async move {
        ws_server_clone.run(listener).await;
    });

    let mut grpc_config = ClientConfig::default();
    grpc_config.enable_metrics = config.grpc.enable_metrics;
    grpc_config.connection_timeout_ms = config.grpc.connection_timeout_ms;
    grpc_config.request_timeout_ms = config.grpc.request_timeout_ms;
    grpc_config.enable_tls = config.grpc.enable_tls;

    let grpc = YellowstoneGrpc::new_with_config(
        config.grpc.endpoint.clone(),
        config.grpc.token.clone(),
        grpc_config,
    )
    .map_err(|e| anyhow::anyhow!("gRPC client creation failed: {}", e))?;

    info!("✅ gRPC client created successfully");

    let protocols = config.get_enabled_protocols();
    if protocols.is_empty() {
        anyhow::bail!("No protocols enabled in config");
    }

    info!("📊 Monitoring protocols: {:?}", protocols);

    let transaction_filter = TransactionFilter::for_protocols(&protocols);
    let account_filter = AccountFilter::for_protocols(&protocols);

    let event_types = config.get_enabled_event_types();
    if event_types.is_empty() {
        anyhow::bail!("No event types enabled in config");
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

    info!("🛑 Press Ctrl+C to stop...");
    tokio::signal::ctrl_c().await?;
    info!("👋 Shutting down gracefully...");

    Ok(())
}