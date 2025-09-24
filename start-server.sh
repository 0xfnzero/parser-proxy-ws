#!/bin/bash

set -e

echo "🚀 Starting Parser Proxy WebSocket Server..."
echo ""

if [ ! -f "config.toml" ]; then
    echo "❌ Error: config.toml not found!"
    echo "Please create a config.toml file first."
    exit 1
fi

echo "📋 Checking configuration..."
echo "   Config file: config.toml"
echo ""

echo "🔨 Building project in release mode..."
cargo build --release
echo ""

echo "✅ Starting server..."
echo "   Press Ctrl+C to stop"
echo ""
echo "================================================"
echo ""

RUST_LOG=${RUST_LOG:-info} ./target/release/parser-proxy-ws