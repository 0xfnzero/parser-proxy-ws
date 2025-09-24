<div align="center">
    <h1>⚡ Parser Proxy WebSocket</h1>
    <h3><em>Ultra-low latency Solana DEX event WebSocket proxy with real-time streaming</em></h3>
</div>

<p align="center">
    <strong>High-performance WebSocket server for streaming Solana DEX events with microsecond-level latency</strong>
</p>

<p align="center">
    <a href="https://github.com/0xfnzero/parser-proxy-ws">
        <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version">
    </a>
    <a href="https://github.com/0xfnzero/parser-proxy-ws/blob/main/LICENSE">
        <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
    </a>
</p>

<p align="center">
    <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
    <img src="https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white" alt="Solana">
    <img src="https://img.shields.io/badge/WebSocket-010101?style=for-the-badge&logo=socket.io&logoColor=white" alt="WebSocket">
    <img src="https://img.shields.io/badge/gRPC-4285F4?style=for-the-badge&logo=grpc&logoColor=white" alt="gRPC">
</p>

<p align="center">
    <a href="https://github.com/0xfnzero/parser-proxy-ws/blob/main/README_CN.md">中文</a> |
    <a href="https://github.com/0xfnzero/parser-proxy-ws/blob/main/README.md">English</a> |
    <a href="https://fnzero.dev/">Website</a> |
    <a href="https://t.me/fnzero_group">Telegram</a> |
    <a href="https://discord.gg/vuazbGkqQE">Discord</a>
</p>

---

A WebSocket proxy server for Solana DEX events based on [sol-parser-sdk](./sol-parser-sdk). Real-time monitoring of DEX trading events from PumpFun, Raydium, Orca, and more, streamed to clients via WebSocket.

## ✨ Features

- 🔥 **Ultra-low Latency** - Built on sol-parser-sdk with 10-20μs parsing latency
- 🌐 **Real-time WebSocket Push** - Support multiple concurrent client connections
- 📊 **Multi-protocol Support** - PumpFun, Raydium AMM V4/CLMM/CPMM, Orca Whirlpool, Meteora, etc.
- 🎯 **Event Filtering** - Only push events you care about
- ⚙️ **TOML Configuration** - All parameters flexibly configurable via config file
- 🔄 **Auto Reconnection** - Automatic client cleanup on disconnect
- 📝 **Multi-language Clients** - Examples in HTML, JavaScript, TypeScript, Python

## 🏗️ Architecture

```
┌─────────────────┐
│ Yellowstone gRPC│
│   (Solana RPC)  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ sol-parser-sdk  │
│  (10-20μs parse)│
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  WebSocket Server│
│  (Broadcast)    │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────┐
│  Clients (HTML/JS/Python/...)       │
│  (Real-time DEX Events)             │
└─────────────────────────────────────┘
```

## 📦 Installation

### 1. Clone Repository

```bash
git clone https://github.com/0xfnzero/parser-proxy-ws.git
cd parser-proxy-ws
```

### 2. Build Project

```bash
cargo build --release
```

## 🚀 Quick Start

### 1. Configure Server

Edit `config.toml`:

```toml
[server]
host = "127.0.0.1"
port = 9001

[grpc]
endpoint = "https://solana-yellowstone-grpc.publicnode.com:443"
token = ""  # Optional: your gRPC token
enable_metrics = true
enable_tls = true
connection_timeout_ms = 10000
request_timeout_ms = 30000

[protocols]
pumpfun = true
raydium_amm_v4 = true
raydium_clmm = true
orca_whirlpool = true
# ... more protocols

[events]
pumpfun_trade = true
pumpfun_create = true
raydium_amm_v4_swap = true
# ... more event types
```

### 2. Start Server

**Method 1: Using Script (Recommended)**

```bash
./start-server.sh
```

**Method 2: Direct Run**

```bash
cargo run --release
```

Server will start based on config file (default `ws://127.0.0.1:9001`)

### 3. Start Client

**Method 1: Using Script (Recommended)**

```bash
./start-client.sh
```

The script will prompt you to choose client type:
- **HTML Client** (Browser visualization, auto-convert Pubkey/Signature)
- **TypeScript Client** (Terminal, auto-convert Pubkey/Signature)
- **Python Client** (Terminal, auto-convert Pubkey/Signature + latency display)

**Method 2: Manual Start**

#### HTML Client (Recommended)

```bash
open examples/client.html
```

✨ **Features**:
- Auto-convert Pubkey/Signature to base58 strings
- Real-time latency display (gRPC → Client)
- Color-coded latency (🟢 < 50ms, 🟡 < 100ms, 🔴 ≥ 100ms)
- ⏸️ **Pause/Resume** - Pause event updates to inspect details

#### TypeScript Client

```bash
cd examples
npm install
npm start
```

✨ **Features**:
- Auto-convert using @solana/web3.js and bs58
- Terminal real-time latency display (with colors)
- TypeScript type safety

#### Python Client

```bash
pip3 install websockets base58
python3 examples/client.py
```

✨ **Features**:
- Auto-convert Pubkey/Signature using `base58` library
- Terminal color latency display
- Microsecond precision latency calculation

## 📊 Supported Protocols and Events

### Supported DEX Protocols

| Protocol | Config Field | Description |
|----------|-------------|-------------|
| PumpFun | `protocols.pumpfun` | PumpFun Meme token trading |
| PumpSwap | `protocols.pumpswap` | PumpSwap exchange protocol |
| Bonk | `protocols.bonk` | Bonk launch platform |
| Raydium AMM V4 | `protocols.raydium_amm_v4` | Raydium Automated Market Maker V4 |
| Raydium CLMM | `protocols.raydium_clmm` | Raydium Concentrated Liquidity MM |
| Raydium CPMM | `protocols.raydium_cpmm` | Raydium Constant Product MM |
| Orca Whirlpool | `protocols.orca_whirlpool` | Orca Whirlpool Concentrated Liquidity |
| Meteora AMM | `protocols.meteora_amm` | Meteora Dynamic AMM |
| Meteora DAMM | `protocols.meteora_damm` | Meteora Dynamic AMM V2 |
| Meteora DLMM | `protocols.meteora_dlmm` | Meteora Dynamic Liquidity MM |

### Supported Event Types

Each protocol supports the following event types (enable in `[events]` config section):

- 📈 **Trade/Swap** - Trading/swapping events
- 💧 **Deposit** - Add liquidity
- 💸 **Withdraw** - Remove liquidity
- 🏊 **Create/Initialize** - Pool creation/initialization

See `config.toml` for complete event list.

## 🔧 Configuration

### Server Configuration

```toml
[server]
host = "127.0.0.1"     # WebSocket listen address
port = 9001             # WebSocket listen port
```

### gRPC Configuration

```toml
[grpc]
endpoint = "https://solana-yellowstone-grpc.publicnode.com:443"
token = ""                    # gRPC auth token (optional)
enable_metrics = true         # Enable performance metrics
enable_tls = true             # Enable TLS
connection_timeout_ms = 10000 # Connection timeout (ms)
request_timeout_ms = 30000    # Request timeout (ms)
```

### Protocol and Event Filtering

Enable/disable specific protocols and events via config:

```toml
[protocols]
pumpfun = true           # Enable PumpFun protocol
raydium_amm_v4 = false   # Disable Raydium AMM V4

[events]
pumpfun_trade = true     # Enable PumpFun trade events
pumpfun_create = false   # Disable PumpFun create events
```

> **Note**: At least one protocol and one event type must be enabled, otherwise server will fail to start

## 📡 WebSocket Message Format

### Pubkey and Signature Format

**Raw data from server**:
- Pubkey: 32-byte array
- Signature: 64-byte array

```json
{
  "signature": [188, 230, 16, ...],  // 64 bytes
  "mint": [208, 230, 16, ...],       // 32 bytes
  "user": [123, 45, 67, ...]         // 32 bytes
}
```

**Client display format**:
- ✅ **HTML/TypeScript Client**: Auto-convert to base58 strings
  ```json
  {
    "signature": "5Jb7XqKGPj8XqKGPj8XqKGPj8XqKGPj...",
    "mint": "D0e610e3f14ed2960e95230e9d74e471...",
    "user": "7xKGPj8XqKGPj8XqKGPj8XqKGPj8Xq..."
  }
  ```
- ✅ **Python Client**: Auto-convert using base58 library

### Event Format

```json
{
  "PumpFunTrade": {
    "metadata": {
      "signature": [byte array],
      "slot": 123456,
      "block_time": 1234567890,
      "grpc_recv_us": 1234567890123456
    },
    "mint": [byte array],
    "token_amount": 1000000,
    "sol_amount": 500000,
    "is_buy": true,
    "user": [byte array],
    "virtual_sol_reserves": 1000000000,
    "virtual_token_reserves": 1000000000
  }
}
```

### Event Types

Each event is wrapped in its corresponding type field:

- `PumpFunTrade` - PumpFun trade
- `PumpFunCreate` - PumpFun creation
- `RaydiumAmmV4Swap` - Raydium AMM swap
- `RaydiumClmmSwap` - Raydium CLMM swap
- `OrcaWhirlpoolSwap` - Orca swap

## 🔬 Development

### Project Structure

```
parser-proxy-ws/
├── src/
│   ├── main.rs           # Main entry point
│   ├── config.rs         # Config file loading and parsing
│   └── ws_server.rs      # WebSocket server implementation
├── examples/
│   ├── client.html       # HTML visualization client
│   ├── client.ts         # TypeScript client example
│   └── client.py         # Python client example
├── sol-parser-sdk/       # Solana parser SDK (submodule)
├── config.toml           # Configuration file
├── Cargo.toml            # Rust project config
└── README.md
```

### Run Tests

```bash
cargo test
```

### Performance Monitoring

Performance metrics are displayed when server starts:

```
📊 gRPC receive time: 1234567890 μs
⚡ Event parsing took: 15 μs
```

## 🎯 Use Cases

1. **Trading Bots** - Real-time DEX monitoring for arbitrage strategies
2. **Price Monitoring** - Track trading prices of specific tokens
3. **New Token Alerts** - Monitor new token creation events
4. **Liquidity Analysis** - Analyze DEX liquidity changes
5. **Data Analytics** - Collect on-chain trading data for analysis

## 📄 License

MIT License

## 🔗 Related Projects

- [sol-parser-sdk](./sol-parser-sdk) - Solana DEX event parsing library
- [Yellowstone gRPC](https://github.com/rpcpool/yellowstone-grpc) - Solana gRPC interface

## 📞 Contact

For issues or suggestions, please submit an Issue.