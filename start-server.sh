#!/bin/bash

set -e

echo "🚀 Starting Parser Proxy WebSocket Server..."
echo ""

if [ ! -f "config.toml" ]; then
    echo "❌ Error: config.toml not found!"
    echo "Please create a config.toml file first."
    exit 1
fi

echo "🔍 Checking for existing parser-proxy-ws processes..."
EXISTING_PIDS=$(pgrep -f "parser-proxy-ws" || true)
if [ ! -z "$EXISTING_PIDS" ]; then
    echo "⚠️  Found existing parser-proxy-ws processes: $EXISTING_PIDS"
    echo "🔪 Killing existing processes..."
    for pid in $EXISTING_PIDS; do
        if sudo -n kill $pid 2>/dev/null; then
            echo "   ✅ Killed process $pid"
        elif kill $pid 2>/dev/null; then
            echo "   ✅ Killed process $pid"
        else
            echo "   ❌ Failed to kill process $pid (permission denied)"
            echo "   Please run: sudo kill $pid"
            exit 1
        fi
    done
    echo "   Waiting 2 seconds for cleanup..."
    sleep 2
else
    echo "   ✅ No existing processes found"
fi
echo ""

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