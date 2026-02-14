#!/bin/bash
set -euo pipefail

# Deployment script for Freenet-web

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CONTRACT_WASM="$PROJECT_ROOT/target/wasm32-unknown-unknown/release/web_container_contract.wasm"
PUBLIC_KEY="$PROJECT_ROOT/build/public_key"
WEBAPP_ARCHIVE="$PROJECT_ROOT/build/web-content.tar.xz"
WEBAPP_METADATA="$PROJECT_ROOT/build/webapp.metadata"
NODE_PORT=7509
MODE="${1:-local}" # 'local' or 'real'

echo "=== Freenet-web Deployment ($MODE) ==="

# 1. Prerequisites
echo "[1/4] Checking prerequisites..."
if ! command -v fdev &>/dev/null; then
    echo "ERROR: fdev not found in PATH"
    exit 1
fi

if [ ! -f "$CONTRACT_WASM" ]; then
    echo "ERROR: Contract WASM not found at $CONTRACT_WASM"
    echo "Run 'cargo make build-web-container' first."
    exit 1
fi

if [ ! -f "$PUBLIC_KEY" ]; then
    echo "ERROR: Public key not found at $PUBLIC_KEY"
    echo "Run 'cargo make sign-webapp' first."
    exit 1
fi

# Check Node Status
node_status=$(curl -s -o /dev/null -w "%{http_code}" "http://127.0.0.1:$NODE_PORT/" 2>/dev/null || echo "000")
if [ "$node_status" = "000" ]; then
    echo "ERROR: Freenet node not responding on port $NODE_PORT"
    echo "Start it: freenet local"
    exit 1
fi
echo "  Node responding (HTTP $node_status)."

# 2. Compute Contract ID
echo "[2/4] Computing Contract ID..."
CONTRACT_ID=$(fdev get-contract-id --code "$CONTRACT_WASM" --parameters "$PUBLIC_KEY")
echo "  Contract ID: $CONTRACT_ID"

# 3. Publish
echo "[3/4] Publishing to network..."
PUBLISH_CMD="fdev network publish"
RELEASE_FLAG=""

if [ "$MODE" == "real" ]; then
    echo "  Publishing to REAL network (propagation may take time)..."
    RELEASE_FLAG="--release"
else
    echo "  Publishing to LOCAL node..."
fi

$PUBLISH_CMD \
    $RELEASE_FLAG \
    --code "$CONTRACT_WASM" \
    --parameters "$PUBLIC_KEY" \
    contract \
    --webapp-archive "$WEBAPP_ARCHIVE" \
    --webapp-metadata "$WEBAPP_METADATA"

# 4. Success
echo ""
echo "[4/4] Deployment Complete!"
echo "=========================================="
echo "Access your site at:"
echo "http://127.0.0.1:$NODE_PORT/v1/contract/web/$CONTRACT_ID/"
echo "=========================================="
