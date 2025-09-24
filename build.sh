#!/bin/bash

# 创建新的 target 目录（如果权限有问题）
if [ ! -w "target" ] 2>/dev/null; then
    echo "⚠️  Target directory has permission issues, using CARGO_TARGET_DIR"
    export CARGO_TARGET_DIR="$PWD/build-output"
    mkdir -p "$CARGO_TARGET_DIR"
fi

echo "🔨 Building parser-proxy-ws..."
cargo build --release "$@"

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"

    # 找到二进制文件位置
    if [ -n "$CARGO_TARGET_DIR" ]; then
        BINARY="$CARGO_TARGET_DIR/release/parser-proxy-ws"
    else
        BINARY="target/release/parser-proxy-ws"
    fi

    if [ -f "$BINARY" ]; then
        echo "📦 Binary location: $BINARY"
    fi
else
    echo "❌ Build failed!"
    exit 1
fi