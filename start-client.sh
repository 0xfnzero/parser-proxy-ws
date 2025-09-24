#!/bin/bash

set -e

echo "🌐 Parser Proxy WebSocket Client Launcher"
echo ""
echo "Please select a client to start:"
echo ""
echo "  1) HTML Client (Browser - Visual UI with Pubkey conversion)"
echo "  2) TypeScript Client (Terminal - with Pubkey conversion)"
echo "  3) Python Client (Terminal - with Pubkey conversion)"
echo ""
read -p "Enter your choice [1-3]: " choice

case $choice in
    1)
        echo ""
        echo "🌐 Opening HTML Client in browser..."
        if [ -f "examples/client.html" ]; then
            if command -v open &> /dev/null; then
                open examples/client.html
            elif command -v xdg-open &> /dev/null; then
                xdg-open examples/client.html
            else
                echo "ℹ️  Please open examples/client.html in your browser manually"
            fi
            echo "✅ HTML Client opened"
            echo "ℹ️  This client automatically converts Pubkey arrays to base58 strings"
        else
            echo "❌ Error: examples/client.html not found!"
            exit 1
        fi
        ;;
    2)
        echo ""
        echo "📦 Starting TypeScript Client..."
        if ! command -v node &> /dev/null; then
            echo "❌ Error: Node.js is not installed!"
            echo "Please install Node.js first: https://nodejs.org/"
            exit 1
        fi

        if ! command -v npm &> /dev/null; then
            echo "❌ Error: npm is not installed!"
            exit 1
        fi

        cd examples
        echo "📥 Installing dependencies..."
        npm install --silent

        if [ -f "client.ts" ]; then
            echo "✅ Starting TypeScript client..."
            echo "   Press Ctrl+C to stop"
            echo ""
            echo "================================================"
            echo ""
            npm start
        else
            echo "❌ Error: client.ts not found!"
            exit 1
        fi
        ;;
    3)
        echo ""
        echo "🐍 Starting Python Client..."
        if ! command -v python3 &> /dev/null; then
            echo "❌ Error: Python 3 is not installed!"
            echo "Please install Python 3 first: https://www.python.org/"
            exit 1
        fi

        echo "📥 Checking dependencies..."
        if ! python3 -c "import websockets" 2>/dev/null; then
            echo "Installing websockets package..."
            pip3 install websockets --quiet
        fi

        if ! python3 -c "import base58" 2>/dev/null; then
            echo "Installing base58 package..."
            pip3 install base58 --quiet
        fi

        if [ -f "examples/client.py" ]; then
            echo "✅ Starting Python client..."
            echo "✨ This client converts Pubkey/Signature to base58 strings"
            echo "   Press Ctrl+C to stop"
            echo ""
            echo "================================================"
            echo ""
            python3 examples/client.py
        else
            echo "❌ Error: examples/client.py not found!"
            exit 1
        fi
        ;;
    *)
        echo "❌ Invalid choice. Please run the script again and select 1, 2, or 3."
        exit 1
        ;;
esac