#!/usr/bin/env python3

import asyncio
import websockets
import json
import time
from datetime import datetime
from base58 import b58encode

def array_to_base58(arr):
    """将字节数组转换为 base58 字符串"""
    try:
        return b58encode(bytes(arr)).decode('utf-8')
    except:
        return ','.join(str(x) for x in arr)

def convert_arrays_in_object(obj, path=''):
    """递归转换对象中的所有 Pubkey 和 Signature 数组"""
    if isinstance(obj, list):
        # 检查是否是 Signature (64 字节数组)
        if len(obj) == 64 and all(isinstance(x, int) and 0 <= x <= 255 for x in obj):
            if path.endswith('.signature') or path == 'signature':
                return array_to_base58(obj)

        # 检查是否是 Pubkey (32 字节数组)
        if len(obj) == 32 and all(isinstance(x, int) and 0 <= x <= 255 for x in obj):
            return array_to_base58(obj)

        return [convert_arrays_in_object(item, f"{path}[{idx}]") for idx, item in enumerate(obj)]

    elif isinstance(obj, dict):
        converted = {}
        for key, value in obj.items():
            new_path = f"{path}.{key}" if path else key
            converted[key] = convert_arrays_in_object(value, new_path)
        return converted

    return obj

async def listen_events():
    uri = "ws://127.0.0.1:9001"

    print("🚀 Connecting to WebSocket server...")

    try:
        async with websockets.connect(uri) as websocket:
            print("✅ Connected to WebSocket server")
            print("📡 Listening for DEX events...\n")

            async for message in websocket:
                try:
                    # 客户端接收时间（微秒）
                    client_recv_us = int(time.time() * 1_000_000)

                    raw_event = json.loads(message)

                    # 转换所有 Pubkey 和 Signature 数组为字符串
                    event = convert_arrays_in_object(raw_event)

                    # 计算延迟
                    event_data = list(event.values())[0] if event else {}
                    grpc_recv_us = event_data.get('metadata', {}).get('grpc_recv_us')

                    print("=" * 80)
                    print(f"📊 New Event Received: {datetime.now().isoformat()}")

                    if grpc_recv_us:
                        latency_us = max(0, client_recv_us - grpc_recv_us)
                        latency_ms = latency_us / 1000

                        # 颜色标识
                        if latency_us < 50000:
                            color = '\033[32m'  # 绿色
                        elif latency_us < 100000:
                            color = '\033[33m'  # 黄色
                        else:
                            color = '\033[31m'  # 红色

                        print(f"⏱️  gRPC Receive Time: {grpc_recv_us} μs")
                        print(f"⏱️  Client Receive Time: {client_recv_us} μs")
                        print(f"{color}⚡ Total Latency: {latency_ms:.2f} ms ({latency_us} μs)\033[0m")

                    print("=" * 80)

                    if "PumpFunTrade" in event:
                        print("🔥 Event Type: PumpFun Trade")
                        print(json.dumps(event["PumpFunTrade"], indent=2))
                    elif "PumpFunCreate" in event:
                        print("🆕 Event Type: PumpFun Create")
                        print(json.dumps(event["PumpFunCreate"], indent=2))
                    elif "RaydiumAmmV4Swap" in event:
                        print("💧 Event Type: Raydium AMM V4 Swap")
                        print(json.dumps(event["RaydiumAmmV4Swap"], indent=2))
                    elif "RaydiumClmmSwap" in event:
                        print("💦 Event Type: Raydium CLMM Swap")
                        print(json.dumps(event["RaydiumClmmSwap"], indent=2))
                    elif "OrcaWhirlpoolSwap" in event:
                        print("🌊 Event Type: Orca Whirlpool Swap")
                        print(json.dumps(event["OrcaWhirlpoolSwap"], indent=2))
                    else:
                        print("❓ Unknown Event Type")
                        print(json.dumps(event, indent=2))

                    print()

                except json.JSONDecodeError as e:
                    print(f"❌ Failed to parse message: {e}")

    except websockets.exceptions.WebSocketException as e:
        print(f"❌ WebSocket error: {e}")
    except KeyboardInterrupt:
        print("\n👋 Closing connection...")

if __name__ == "__main__":
    asyncio.run(listen_events())