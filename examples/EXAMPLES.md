# Parser Proxy WebSocket Library Examples

This directory contains examples showing different ways to use `parser-proxy-ws` as a library in your Rust applications.

## Examples Overview

### 1. `simple_usage.rs` - Basic Library Usage
The simplest way to use parser-proxy-ws. Just create a server and start it.

```bash
cargo run --example simple_usage
```

**Use case**: Standalone WebSocket server for DEX events

### 2. `advanced_usage.rs` - Advanced Configuration
Shows more control over server lifecycle, custom logging, and graceful shutdown.

```bash
cargo run --example advanced_usage
```

**Use case**: Custom applications needing fine-grained control

### 3. `server_integration.rs` - Full Server Integration ⭐
**This is the pattern used in the main trading-proxy-http server.**

Shows how to run both HTTP API and WebSocket servers in parallel within a single application.

```bash
cargo run --example server_integration
```

**Use case**: Exactly like the main server - provides both HTTP trading API and WebSocket event streaming

### 4. `integrated_server.rs` - Mock Integration
Simplified version showing the integration pattern with mock HTTP server.

```bash
cargo run --example integrated_server
```

**Use case**: Understanding the integration architecture

### 5. `library_integration.rs` - Custom Application Integration
Shows how to integrate parser-proxy-ws into your own trading application with custom business logic.

```bash
cargo run --example library_integration
```

**Use case**: Building custom trading applications

### 6. `trading_bot_integration.rs` - Trading Bot Pattern
Demonstrates the pattern for trading bots that need both event streaming and business logic.

```bash
cargo run --example trading_bot_integration
```

**Use case**: Trading bots and automated systems

## Configuration

All examples expect a `config.toml` file in the current directory or specified via environment variables:

```bash
# Use a specific config file
PARSER_CONFIG_PATH=path/to/your/config.toml cargo run --example server_integration

# Use a specific HTTP port (for examples that include HTTP servers)
HTTP_PORT=8080 cargo run --example server_integration
```

## Key Integration Patterns

### Pattern 1: Simple Integration (Examples 1, 2)
```rust
use parser_proxy_ws::ParserProxyServer;

let server = ParserProxyServer::new("config.toml")?;
server.start().await?;
```

### Pattern 2: Parallel Services (Examples 3, 4) ⭐ **Main Server Pattern**
```rust
// Create both servers
let http_server = HttpServer::new(port);
let ws_server = WebSocketServer::new(config_path)?;

// Start in parallel
let http_handle = tokio::spawn(async move {
    http_server.start().await
});

let ws_handle = tokio::spawn(async move {
    ws_server.start().await
});

// Wait for shutdown
tokio::signal::ctrl_c().await?;
http_handle.abort();
ws_handle.abort();
```

### Pattern 3: Custom Application (Examples 5, 6)
```rust
// Start WebSocket server in background
let server = ParserProxyServer::new("config.toml")?;
tokio::spawn(async move {
    server.start().await
});

// Run your custom business logic
run_trading_logic().await?;
```

## Environment Variables

- `PARSER_CONFIG_PATH`: Path to parser-proxy-ws configuration file
- `HTTP_PORT`: Port for HTTP server (in examples that include HTTP)
- `RUST_LOG`: Logging level (e.g., `info`, `debug`, `parser_proxy_ws=debug`)

## Dependencies

These examples use:
- `parser_proxy_ws` - The main library
- `tokio` - Async runtime
- `tracing` - Logging
- `anyhow` - Error handling
- `dotenv` - Environment variable loading (some examples)

## Real-World Usage

The `server_integration.rs` example shows the **exact pattern** used in the main `trading-proxy-http` server:

1. Load configuration from environment/files
2. Create HTTP server for trading API
3. Create WebSocket server for event streaming
4. Run both servers in parallel
5. Handle graceful shutdown

This pattern allows a single application to provide:
- ✅ HTTP API endpoints for trading operations
- ✅ WebSocket streaming for real-time DEX events
- ✅ Unified configuration and logging
- ✅ Single deployment unit