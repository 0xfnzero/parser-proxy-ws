# 客户端示例

## 🎯 Pubkey 和 Signature 自动转换

所有客户端都会接收到相同的原始数据，但**不同客户端显示格式不同**：

### 原始数据格式（服务器发送）

```json
{
  "PumpFunTrade": {
    "metadata": {
      "signature": [188, 230, 16, 126, ...],  // 64 字节
      "slot": 123456
    },
    "mint": [208, 230, 16, 126, ...],         // 32 字节
    "user": [123, 45, 67, 89, ...]            // 32 字节
  }
}
```

## 📋 客户端对比

| 客户端 | Signature 格式 | Pubkey 格式 | 延迟计算 | 技术栈 | 推荐用途 |
|--------|---------------|------------|---------|--------|---------|
| **client.html** | ✅ base58 字符串 | ✅ base58 字符串 | ✅ 彩色显示 | Solana Web3.js + bs58 | 可视化监控 |
| **client.ts** | ✅ base58 字符串 | ✅ base58 字符串 | ✅ 彩色终端 | TypeScript + bs58 | 开发/调试 |
| **client.js** | ⚠️ 字节数组 | ⚠️ 字节数组 | ❌ 无 | Node.js | 原始数据处理 |
| **client.py** | ✅ base58 字符串 | ✅ base58 字符串 | ✅ 彩色终端 | Python + base58 | 数据分析 |

## 🚀 使用方法

### 1. HTML 客户端（推荐）

```bash
open client.html
```

**特性**：
- ✅ 自动转换 Signature (64 字节) → base58 (自定义实现)
- ✅ 自动转换 Pubkey (32 字节) → base58 (Solana Web3.js)
- ✅ 实时延迟计算（gRPC → 客户端）
- ✅ 延迟颜色标识（绿色 < 50ms，黄色 < 100ms，红色 ≥ 100ms）
- ✅ ⏸️ **暂停/恢复功能** - 暂停事件更新，方便查看详情
- ✅ 可视化界面
- ✅ 无需安装依赖（内置 base58 编码器）

**使用说明**：
- 点击 "暂停" 按钮可暂停事件更新
- 暂停期间新事件会被缓存（最多100个）
- 点击 "恢复" 继续显示新事件（跳过暂停期间的事件）

**显示示例**：
```json
{
  "signature": "5Jb7XqKGPj8XqKGPj8XqKGPj8XqKGPj8XqKGPj8XqKGPj8XqKGPj8XqKGPj...",
  "mint": "D0e610e3f14ed2960e95230e9d74e471d48483f0b1f89545482a2ab32a897212",
  "user": "7xKGPj8XqKGPj8XqKGPj8XqKGPj8XqKGPj8XqKGPj8XqKGPj8XqKGPj8Xq..."
}
```

### 2. TypeScript 客户端

```bash
npm install
npm start
```

**特性**：
- ✅ 使用 `bs58` 库转换 Signature
- ✅ 使用 `@solana/web3.js` 转换 Pubkey
- ✅ 实时延迟计算和显示
- ✅ 终端彩色输出
- ✅ TypeScript 类型安全

**延迟显示**：
```
⏱️  gRPC Receive Time: 1234567890123456 μs
⏱️  Client Receive Time: 1234567890223456 μs
⚡ Total Latency: 100.00 ms (100000 μs)
```

**依赖**：
- `@solana/web3.js`: Solana SDK
- `bs58`: Base58 编码/解码
- `ws`: WebSocket 客户端

### 3. Node.js 客户端（原始数据）

```bash
npm install ws
node client.js
```

**显示示例**（原始字节数组）：
```json
{
  "signature": [188, 230, 16, 126, 63, 20, ...],
  "mint": [208, 230, 16, 126, 63, 20, ...],
  "user": [123, 45, 67, 89, 101, ...]
}
```

### 4. Python 客户端

```bash
pip3 install websockets base58
python3 client.py
```

**特性**：
- ✅ 使用 `base58` 库转换 Signature 和 Pubkey
- ✅ 终端彩色延迟显示
- ✅ 微秒级时间精度

**显示示例**：
```
⏱️  gRPC Receive Time: 1758728602154124 μs
⏱️  Client Receive Time: 1758728602204567 μs
⚡ Total Latency: 50.44 ms (50443 μs)  [彩色]
{
  "signature": "5Jb7XqKGPj8XqKGPj8XqKGPj...",
  "mint": "D0e610e3f14ed2960e95230e9d74e471..."
}
```

## 🔧 转换实现细节

### Python 实现

```python
from base58 import b58encode

def array_to_base58(arr):
    """将字节数组转换为 base58 字符串"""
    return b58encode(bytes(arr)).decode('utf-8')

def convert_arrays_in_object(obj, path=''):
    """递归转换所有 Pubkey 和 Signature"""
    if isinstance(obj, list):
        # Signature (64 字节)
        if len(obj) == 64 and path.endswith('.signature'):
            return array_to_base58(obj)
        # Pubkey (32 字节)
        if len(obj) == 32:
            return array_to_base58(obj)
    # ... 递归处理
```

## 🔧 转换实现细节

### TypeScript 实现

```typescript
// Signature (64 字节) 转换
function signatureArrayToString(arr: number[]): string {
    const buffer = Buffer.from(arr);
    return bs58.encode(buffer);
}

// Pubkey (32 字节) 转换
function pubkeyArrayToString(arr: number[]): string {
    const buffer = Buffer.from(arr);
    return new PublicKey(buffer).toBase58();
}

// 递归转换所有字段
function convertArraysInObject(obj: any, path: string = ''): any {
    if (Array.isArray(obj)) {
        // 检查是否是 Signature (64 字节)
        if (obj.length === 64 && path.endsWith('.signature')) {
            return signatureArrayToString(obj);
        }
        // 检查是否是 Pubkey (32 字节)
        if (obj.length === 32) {
            return pubkeyArrayToString(obj);
        }
    }
    // ... 递归处理嵌套对象
}
```

### HTML 实现

```javascript
// 使用 CDN 加载库
<script src="https://cdn.jsdelivr.net/npm/@solana/web3.js@latest/lib/index.iife.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/bs58@6.0.0/index.min.js"></script>

// Signature 转换
function signatureArrayToString(arr) {
    const uint8Array = new Uint8Array(arr);
    return bs58.encode(uint8Array);
}

// Pubkey 转换
function pubkeyArrayToString(arr) {
    const uint8Array = new Uint8Array(arr);
    return new solanaWeb3.PublicKey(uint8Array).toBase58();
}
```

## 📝 注意事项

1. **字段识别逻辑**：
   - 64 字节数组 + 字段名包含 "signature" → 转换为 Signature
   - 32 字节数组 → 转换为 Pubkey

2. **性能考虑**：
   - 转换在客户端进行，不影响服务器性能
   - HTML/TS 客户端会有轻微延迟（毫秒级）

3. **兼容性**：
   - HTML 客户端需要现代浏览器（支持 ES6+）
   - TS 客户端需要 Node.js 14+

4. **错误处理**：
   - 如果转换失败，会回退到原始数组字符串
   - 不会中断数据流

## ⏱️ 延迟计算

### 计算原理

```
事件流程：
Solana Node → gRPC Server → Parser SDK → WebSocket Server → Client

延迟计算：
Total Latency = Client Receive Time - gRPC Receive Time
```

### TypeScript 实现

```typescript
// 1. 获取客户端接收时间（微秒）- 使用高精度时间
const msTimestamp = Date.now();
const perfMs = performance.now();

// 使用 performance.now() 的小数部分获得亚毫秒精度
const clientRecvUs = Math.floor(msTimestamp * 1000) + Math.floor((perfMs % 1) * 1000);

// 2. 从事件中提取 gRPC 接收时间
const eventData = Object.values(event)[0];
const grpcRecvUs = eventData?.metadata?.grpc_recv_us;

// 3. 计算延迟
const latencyUs = clientRecvUs - grpcRecvUs;
const latencyMs = (latencyUs / 1000).toFixed(2);

// 4. 颜色标识
const color = latencyUs < 50000 ? 'green' :
              latencyUs < 100000 ? 'yellow' : 'red';
```

**注意**：
- `Date.now()` 只提供毫秒精度
- `performance.now()` 提供亚毫秒精度（通常到微秒级）
- 组合使用可获得微秒级的 Unix 时间戳

### HTML 实现

#### Base58 编码（自定义实现）

```javascript
const BASE58_ALPHABET = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';

function base58Encode(bytes) {
    const digits = [0];

    for (let i = 0; i < bytes.length; i++) {
        let carry = bytes[i];
        for (let j = 0; j < digits.length; j++) {
            carry += digits[j] << 8;
            digits[j] = carry % 58;
            carry = (carry / 58) | 0;
        }
        while (carry > 0) {
            digits.push(carry % 58);
            carry = (carry / 58) | 0;
        }
    }

    // 处理前导零
    for (let i = 0; i < bytes.length && bytes[i] === 0; i++) {
        digits.push(0);
    }

    return digits.reverse().map(d => BASE58_ALPHABET[d]).join('');
}

// Signature 转换
function signatureArrayToString(arr) {
    return base58Encode(new Uint8Array(arr));
}

// Pubkey 转换（优先使用 Solana Web3.js）
function pubkeyArrayToString(arr) {
    const uint8Array = new Uint8Array(arr);
    if (typeof solanaWeb3 !== 'undefined') {
        return new solanaWeb3.PublicKey(uint8Array).toBase58();
    }
    return base58Encode(uint8Array);
}
```

#### 延迟计算

```javascript
// 使用高精度时间戳（performance API）
const absoluteMs = performance.timeOrigin + performance.now();
const clientRecvUs = Math.floor(absoluteMs * 1000);

const grpcRecvUs = eventData.metadata.grpc_recv_us;
const latencyUs = clientRecvUs - grpcRecvUs;
const latencyMs = (latencyUs / 1000).toFixed(2);

// 更新状态栏
document.getElementById('latency').textContent = latencyMs + ' ms';

// 在事件卡片上显示彩色标签
const color = latencyMs < 50 ? '#28a745' :
              latencyMs < 100 ? '#ffc107' : '#dc3545';
```

**关键点**：
- 自定义 base58 实现，无需外部依赖
- `performance.timeOrigin`: 页面加载时的 Unix 时间戳（毫秒）
- `performance.now()`: 页面加载后的相对时间（毫秒，高精度）
- 两者相加得到绝对的高精度时间戳

### 延迟范围说明

| 延迟范围 | 颜色 | 说明 |
|---------|------|------|
| < 50 ms | 🟢 绿色 | 优秀 - 网络状况良好 |
| 50-100 ms | 🟡 黄色 | 正常 - 可接受的延迟 |
| ≥ 100 ms | 🔴 红色 | 较高 - 可能网络拥堵 |

**注意**：延迟包含：
- ✅ SDK 解析时间（10-20μs）
- ✅ WebSocket 传输时间
- ✅ 网络延迟
- ✅ 客户端处理时间

**负数延迟问题**：
如果出现负数延迟，通常是以下原因：
1. ❌ **时间精度不足** - `Date.now()` 只有毫秒精度
2. ❌ **时钟不同步** - 服务器和客户端时钟可能有偏差
3. ✅ **解决方案** - 使用 `performance.timeOrigin + performance.now()` 获得微秒级精度

已修复方法：
```javascript
// 旧方法（可能出现负数）
const clientRecvUs = Date.now() * 1000;  // 精度不够

// 新方法（高精度）
const absoluteMs = performance.timeOrigin + performance.now();
const clientRecvUs = Math.floor(absoluteMs * 1000);
```

## 🎨 自定义转换

如果需要转换其他字段，可以修改 `convertArraysInObject` 函数：

```typescript
// 添加自定义字段转换
if (path.endsWith('.custom_field') && obj.length === 64) {
    return customConversion(obj);
}
```