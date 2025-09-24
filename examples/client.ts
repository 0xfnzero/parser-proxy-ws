import WebSocket from 'ws';
import { PublicKey } from '@solana/web3.js';
import bs58 from 'bs58';
import { performance } from 'perf_hooks';

// Pubkey 数组转换为 base58 字符串
function pubkeyArrayToString(arr: number[]): string {
    try {
        const buffer = Buffer.from(arr);
        return new PublicKey(buffer).toBase58();
    } catch (e) {
        return arr.join(',');
    }
}

// Signature 数组转换为 base58 字符串
function signatureArrayToString(arr: number[]): string {
    try {
        const buffer = Buffer.from(arr);
        return bs58.encode(buffer);
    } catch (e) {
        return arr.join(',');
    }
}

// 递归转换对象中的所有 Pubkey 和 Signature 数组
function convertArraysInObject(obj: any, path: string = ''): any {
    if (Array.isArray(obj)) {
        // 检查是否是 Signature (64 字节数组)
        if (obj.length === 64 && obj.every(n => typeof n === 'number' && n >= 0 && n <= 255)) {
            // 如果字段名是 signature，转换为 base58
            if (path.endsWith('.signature') || path === 'signature') {
                return signatureArrayToString(obj);
            }
        }
        // 检查是否是 Pubkey (32 字节数组)
        if (obj.length === 32 && obj.every(n => typeof n === 'number' && n >= 0 && n <= 255)) {
            return pubkeyArrayToString(obj);
        }
        return obj.map((item, idx) => convertArraysInObject(item, `${path}[${idx}]`));
    } else if (obj && typeof obj === 'object') {
        const converted: any = {};
        for (const [key, value] of Object.entries(obj)) {
            const newPath = path ? `${path}.${key}` : key;
            converted[key] = convertArraysInObject(value, newPath);
        }
        return converted;
    }
    return obj;
}

const ws = new WebSocket('ws://127.0.0.1:9001');

ws.on('open', function open() {
    console.log('✅ Connected to WebSocket server');
    console.log('📡 Listening for DEX events...\n');
});

ws.on('message', function message(data: WebSocket.Data) {
    // 使用 performance.now() 获取高精度时间戳（亚毫秒）
    // performance.timeOrigin 是进程启动时的 Unix 时间戳（毫秒）
    // performance.now() 是从进程启动到现在的时间（毫秒，带小数）
    const nowMs = performance.timeOrigin + performance.now();
    const clientRecvUs = Math.floor(nowMs * 1000);

    try {
        const rawEvent = JSON.parse(data.toString());

        // 转换所有 Pubkey 和 Signature 数组为字符串
        const event = convertArraysInObject(rawEvent);

        // 计算延迟
        // 从 metadata 中提取 grpc_recv_us
        const eventData = Object.values(event)[0] as any;
        const grpcRecvUs = eventData?.metadata?.grpc_recv_us as number | undefined;

        console.log('='.repeat(80));
        console.log('📊 New Event Received:', new Date().toISOString());

        if (grpcRecvUs !== undefined) {
            const rawLatencyUs = clientRecvUs - grpcRecvUs;
            const latencyUs = Math.max(0, rawLatencyUs);
            const latencyMs = (latencyUs / 1000).toFixed(2);
            const latencyColor = latencyUs < 50000 ? '\x1b[32m' : latencyUs < 100000 ? '\x1b[33m' : '\x1b[31m';

            // 调试输出：显示原始延迟（可能为负）
            if (rawLatencyUs < 0) {
                console.log(`\x1b[90m⚠️  Raw latency was negative: ${rawLatencyUs} μs (likely clock skew)\x1b[0m`);
            }

            console.log(`⏱️  gRPC Receive Time: ${grpcRecvUs} μs`);
            console.log(`⏱️  Client Receive Time: ${clientRecvUs} μs`);
            console.log(`${latencyColor}⚡ Total Latency: ${latencyMs} ms (${latencyUs} μs)\x1b[0m`);
        }

        console.log('='.repeat(80));

        if (event.PumpFunTrade) {
            console.log('🔥 Event Type: PumpFun Trade');
            console.log(JSON.stringify(event.PumpFunTrade, null, 2));
        } else if (event.PumpFunCreate) {
            console.log('🆕 Event Type: PumpFun Create');
            console.log(JSON.stringify(event.PumpFunCreate, null, 2));
        } else if (event.RaydiumAmmV4Swap) {
            console.log('💧 Event Type: Raydium AMM V4 Swap');
            console.log(JSON.stringify(event.RaydiumAmmV4Swap, null, 2));
        } else if (event.RaydiumClmmSwap) {
            console.log('💦 Event Type: Raydium CLMM Swap');
            console.log(JSON.stringify(event.RaydiumClmmSwap, null, 2));
        } else if (event.OrcaWhirlpoolSwap) {
            console.log('🌊 Event Type: Orca Whirlpool Swap');
            console.log(JSON.stringify(event.OrcaWhirlpoolSwap, null, 2));
        } else {
            console.log('❓ Unknown Event Type');
            console.log(JSON.stringify(event, null, 2));
        }

        console.log('');
    } catch (e: any) {
        console.error('❌ Failed to parse message:', e.message);
    }
});

ws.on('error', function error(err: Error) {
    console.error('❌ WebSocket error:', err.message);
});

ws.on('close', function close() {
    console.log('🔌 Connection closed');
});

process.on('SIGINT', () => {
    console.log('\n👋 Closing connection...');
    ws.close();
    process.exit(0);
});