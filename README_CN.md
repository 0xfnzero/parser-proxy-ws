<div align="center">
    <h1>⚡ Parser Proxy WebSocket</h1>
    <h3><em>超低延迟 Solana DEX 事件 WebSocket 代理服务器</em></h3>
</div>

<p align="center">
    <strong>高性能 WebSocket 服务器，用于实时推送 Solana DEX 事件，微秒级延迟</strong>
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
    <a href="https://github.com/0xfnzero/parser-proxy-ws/blob/main/README.md">English</a> |
    <a href="https://github.com/0xfnzero/parser-proxy-ws/blob/main/README_CN.md">中文</a> |
    <a href="https://fnzero.dev/">Website</a> |
    <a href="https://t.me/fnzero_group">Telegram</a> |
    <a href="https://discord.gg/vuazbGkqQE">Discord</a>
</p>

---

基于 [sol-parser-sdk](./sol-parser-sdk) 的 Solana DEX 事件 WebSocket 代理服务器。实时监听 PumpFun, Raydium, Orca 等 DEX 交易事件，通过 WebSocket 推送给客户端。

## ✨ 特性

- 🔥 **超低延迟** - 基于 sol-parser-sdk 的 10-20μs 解析延迟
- 🌐 **WebSocket 实时推送** - 支持多客户端同时连接
- 📊 **多协议支持** - PumpFun, Raydium AMM V4/CLMM/CPMM, Orca Whirlpool, Meteora 等
- 🎯 **事件过滤** - 仅推送关注的事件类型
- ⚙️ **TOML 配置** - 所有参数可通过配置文件灵活调整
- 🔄 **自动重连** - 客户端断线自动清理
- 📝 **多语言客户端** - 提供 HTML, TypeScript, Python 示例

## 🏗️ 架构

```
┌─────────────────┐
│ Yellowstone gRPC│
│   (Solana RPC)  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ sol-parser-sdk  │
│  (10-20μs 解析) │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  WebSocket 服务器│
│   (广播事件)     │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────┐
│  客户端 (HTML/TS/Python/...)         │
│  (实时接收 DEX 事件)                  │
└─────────────────────────────────────┘
```

## 📦 安装

### 1. 克隆仓库

```bash
git clone https://github.com/0xfnzero/parser-proxy-ws.git
cd parser-proxy-ws
```

### 2. 构建项目

```bash
cargo build --release
```

## 🚀 快速开始

### 1. 配置服务器

编辑 `config.toml` 配置文件：

```toml
[server]
host = "127.0.0.1"
port = 9001

[grpc]
endpoint = "https://solana-yellowstone-grpc.publicnode.com:443"
token = ""  # 可选：填入你的 gRPC token
enable_metrics = true
enable_tls = true
connection_timeout_ms = 10000
request_timeout_ms = 30000

[protocols]
pumpfun = true
raydium_amm_v4 = true
raydium_clmm = true
orca_whirlpool = true
# ... 更多协议

[events]
pumpfun_trade = true
pumpfun_create = true
raydium_amm_v4_swap = true
# ... 更多事件类型
```

### 2. 启动服务器

**方式 1: 使用启动脚本（推荐）**

```bash
./start-server.sh
```

**方式 2: 直接运行**

```bash
cargo run --release
```

服务器将根据配置文件启动（默认 `ws://127.0.0.1:9001`）

### 3. 启动客户端

**方式 1: 使用启动脚本（推荐）**

```bash
./start-client.sh
```

脚本会提示选择客户端类型：
- **HTML 客户端**（浏览器可视化，自动转换 Pubkey/Signature）
- **TypeScript 客户端**（终端，自动转换 Pubkey/Signature）
- **Python 客户端**（终端，自动转换 Pubkey/Signature + 延迟计算）

**方式 2: 手动启动**

#### 使用 HTML 客户端（推荐）

```bash
open examples/client.html
```

✨ **特性**:
- 自动将 Pubkey/Signature 转换为 base58 字符串
- 实时显示延迟（gRPC → 客户端）
- 延迟颜色标识（🟢 < 50ms, 🟡 < 100ms, 🔴 ≥ 100ms）
- ⏸️ **暂停/恢复功能** - 点击暂停按钮可暂停事件更新，方便查看详情

#### 使用 TypeScript 客户端

```bash
cd examples
npm install
npm start
```

✨ **特性**:
- 使用 @solana/web3.js 和 bs58 自动转换
- 终端实时显示延迟（带颜色）
- TypeScript 类型安全

#### 使用 Python 客户端

```bash
pip3 install websockets base58
python3 examples/client.py
```

✨ **特性**:
- 使用 `base58` 库自动转换 Pubkey/Signature
- 终端彩色延迟显示
- 微秒级延迟计算

## 📊 支持的协议和事件类型

### 支持的 DEX 协议

| 协议 | 配置字段 | 说明 |
|------|---------|------|
| PumpFun | `protocols.pumpfun` | PumpFun Meme 币交易 |
| PumpSwap | `protocols.pumpswap` | PumpSwap 交换协议 |
| Bonk | `protocols.bonk` | Bonk 发射平台 |
| Raydium AMM V4 | `protocols.raydium_amm_v4` | Raydium 自动做市商 V4 |
| Raydium CLMM | `protocols.raydium_clmm` | Raydium 集中流动性做市 |
| Raydium CPMM | `protocols.raydium_cpmm` | Raydium 恒定乘积做市 |
| Orca Whirlpool | `protocols.orca_whirlpool` | Orca Whirlpool 集中流动性 |
| Meteora AMM | `protocols.meteora_amm` | Meteora 动态 AMM |
| Meteora DAMM | `protocols.meteora_damm` | Meteora 动态 AMM V2 |
| Meteora DLMM | `protocols.meteora_dlmm` | Meteora 动态流动性做市 |

### 支持的事件类型

每个协议支持以下事件类型（在 `[events]` 配置段中启用）：

- 📈 **Trade/Swap** - 交易/兑换事件
- 💧 **Deposit** - 添加流动性
- 💸 **Withdraw** - 移除流动性
- 🏊 **Create/Initialize** - 池创建/初始化

完整事件列表请参考 `config.toml` 文件。

## 🔧 配置说明

### 服务器配置

```toml
[server]
host = "127.0.0.1"     # WebSocket 监听地址
port = 9001             # WebSocket 监听端口
```

### gRPC 配置

```toml
[grpc]
endpoint = "https://solana-yellowstone-grpc.publicnode.com:443"
token = ""                    # gRPC 认证 token（可选）
enable_metrics = true         # 启用性能指标
enable_tls = true             # 启用 TLS
connection_timeout_ms = 10000 # 连接超时（毫秒）
request_timeout_ms = 30000    # 请求超时（毫秒）
```

### 协议和事件过滤

通过配置文件启用/禁用特定协议和事件：

```toml
[protocols]
pumpfun = true           # 启用 PumpFun 协议
raydium_amm_v4 = false   # 禁用 Raydium AMM V4

[events]
pumpfun_trade = true     # 启用 PumpFun 交易事件
pumpfun_create = false   # 禁用 PumpFun 创建事件
```

> **注意**: 至少需要启用一个协议和一个事件类型，否则服务器启动会失败

## 📡 WebSocket 消息格式

### Pubkey 和 Signature 格式说明

**服务器发送的原始数据**：
- Pubkey: 32 字节数组
- Signature: 64 字节数组

```json
{
  "signature": [188, 230, 16, ...],  // 64 字节
  "mint": [208, 230, 16, ...],       // 32 字节
  "user": [123, 45, 67, ...]         // 32 字节
}
```

**客户端显示格式**：
- ✅ **HTML/TypeScript 客户端**：自动转换为 base58 字符串
  ```json
  {
    "signature": "5Jb7XqKGPj8XqKGPj8XqKGPj8XqKGPj...",
    "mint": "D0e610e3f14ed2960e95230e9d74e471...",
    "user": "7xKGPj8XqKGPj8XqKGPj8XqKGPj8Xq..."
  }
  ```
- ✅ **Python 客户端**：使用 base58 库自动转换

### 事件格式

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

### 事件类型

每个事件都包含在对应的类型字段中：

- `PumpFunTrade` - PumpFun 交易
- `PumpFunCreate` - PumpFun 创建
- `RaydiumAmmV4Swap` - Raydium AMM 交换
- `RaydiumClmmSwap` - Raydium CLMM 交换
- `OrcaWhirlpoolSwap` - Orca 交换

## 🔬 开发

### 项目结构

```
parser-proxy-ws/
├── src/
│   ├── main.rs           # 主程序入口
│   ├── config.rs         # 配置文件加载和解析
│   └── ws_server.rs      # WebSocket 服务器实现
├── examples/
│   ├── client.html       # HTML 可视化客户端
│   ├── client.ts         # TypeScript 客户端示例
│   └── client.py         # Python 客户端示例
├── sol-parser-sdk/       # Solana 解析 SDK (子模块)
├── config.toml           # 配置文件
├── Cargo.toml            # Rust 项目配置
└── README.md
```

### 运行测试

```bash
cargo test
```

### 性能监控

启动服务器时会显示性能指标：

```
📊 gRPC 接收时间: 1234567890 μs
⚡ 事件解析耗时: 15 μs
```

## 🎯 使用场景

1. **交易机器人** - 实时监控 DEX 交易，执行套利策略
2. **价格监控** - 跟踪特定代币的交易价格
3. **新币监控** - 监听新代币创建事件
4. **流动性分析** - 分析 DEX 流动性变化
5. **数据分析** - 收集链上交易数据进行分析

## 📄 许可证

MIT License

## 🔗 相关项目

- [sol-parser-sdk](./sol-parser-sdk) - Solana DEX 事件解析库
- [Yellowstone gRPC](https://github.com/rpcpool/yellowstone-grpc) - Solana gRPC 接口

## 📞 联系方式

如有问题或建议，请提交 Issue。