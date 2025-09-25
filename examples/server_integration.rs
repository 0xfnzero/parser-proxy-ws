//! # Server Integration Example
//!
//! This example demonstrates exactly how parser-proxy-ws is integrated into the
//! main trading-proxy-http server. It shows the pattern of running both HTTP and
//! WebSocket servers in parallel within a single application.

use parser_proxy_ws::{ParserProxyServer, Config};
use tokio::spawn;
use tracing::{info, error};
use std::sync::Arc;

// Mock types to simulate the actual server structure
#[derive(Debug)]
struct AppConfig {
    pub http_port: u16,
    pub parser_config_path: String,
}

impl AppConfig {
    fn load() -> Self {
        Self {
            http_port: std::env::var("HTTP_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            parser_config_path: std::env::var("PARSER_CONFIG_PATH")
                .unwrap_or_else(|_| "parser-proxy-ws/config.toml".to_string()),
        }
    }
}

// Mock HTTP server (in the real server, this would be axum-based)
struct HttpServer {
    port: u16,
}

impl HttpServer {
    fn new(port: u16) -> Self {
        Self { port }
    }

    async fn start(self) -> anyhow::Result<()> {
        info!("🌐 HTTP API server listening on port {}", self.port);
        info!("📋 Available endpoints:");
        info!("   GET  /health      - Health check");
        info!("   POST /api/buy     - Buy tokens");
        info!("   POST /api/sell    - Sell tokens");

        // Simulate HTTP server running
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            info!("📡 HTTP server health check: OK");
        }
    }
}

// WebSocket server wrapper (exactly as used in the main server)
struct WebSocketServer {
    parser_server: ParserProxyServer,
}

impl WebSocketServer {
    fn new<P: AsRef<std::path::Path>>(config_path: P) -> anyhow::Result<Self> {
        let parser_server = ParserProxyServer::new(config_path)?;
        Ok(Self { parser_server })
    }

    async fn start(self) -> anyhow::Result<()> {
        info!("🔗 Starting integrated WebSocket server...");
        self.parser_server.start().await
    }

    fn ws_url(&self) -> String {
        self.parser_server.ws_url()
    }

    fn config(&self) -> &Config {
        self.parser_server.config()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if present (like in the main server)
    if let Err(e) = dotenv::dotenv() {
        eprintln!("⚠️ 未找到 .env 文件或加载失败: {}", e);
        eprintln!("将使用环境变量或默认配置");
    } else {
        println!("✅ 已加载 .env 文件");
    }

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,parser_proxy_ws=debug")
        .init();

    println!("🚀 启动集成交易代理服务 (HTTP + WebSocket)...");

    // Load configuration
    let config = AppConfig::load();
    info!("⚙️  Configuration loaded: {:#?}", config);

    // Create servers
    let http_server = HttpServer::new(config.http_port);
    let ws_server = WebSocketServer::new(&config.parser_config_path)?;

    println!("🌐 HTTP API 服务器端口: {}", config.http_port);
    println!("🔗 WebSocket 配置文件: {}", config.parser_config_path);
    println!("📡 WebSocket URL: {}", ws_server.ws_url());

    // Display enabled protocols
    let enabled_protocols = ws_server.config().get_enabled_protocols();
    if enabled_protocols.is_empty() {
        info!("⚠️  No DEX protocols enabled - check your configuration");
    } else {
        info!("📊 Monitoring DEX protocols: {:?}", enabled_protocols);
    }

    // Start both servers in parallel (exactly as in main server)
    let http_handle = spawn(async move {
        if let Err(e) = http_server.start().await {
            error!("❌ HTTP 服务器错误: {}", e);
        }
    });

    let ws_handle = spawn(async move {
        if let Err(e) = ws_server.start().await {
            error!("❌ WebSocket 服务器错误: {}", e);
        }
    });

    println!("✅ 服务器启动成功! 等待连接...");
    println!("🔗 客户端现在可以:");
    println!("   - 连接到 HTTP API 执行交易操作");
    println!("   - 连接到 WebSocket 接收实时 DEX 事件");
    println!("🛑 按 Ctrl+C 停止服务器");

    // Wait for shutdown signal (exactly as in main server)
    tokio::signal::ctrl_c().await?;
    println!("👋 正在优雅关闭服务器...");

    // Terminate both servers
    http_handle.abort();
    ws_handle.abort();

    // Allow time for cleanup
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    println!("✅ 关闭完成 - HTTP 和 WebSocket 服务器已停止");

    Ok(())
}