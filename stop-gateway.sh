#!/bin/bash

# 关闭 charge-mgt-gateway 服务

set -e
cd "$(dirname "$0")"

BINARY="target/debug/charge_mgt_gateway"
NAME="charge-mgt-gateway"

pid=$(pgrep -f "$BINARY" 2>/dev/null | head -1)
if [ -z "$pid" ]; then
    echo "$NAME: not running"
    exit 0
fi

echo "Stopping $NAME (PID $pid)..."
kill "$pid" 2>/dev/null || true
for _ in $(seq 1 5); do
    if ! kill -0 "$pid" 2>/dev/null; then
        echo "✓ $NAME stopped"
        exit 0
    fi
    sleep 1
done

echo "! $NAME did not exit in 5s, force killing..."
kill -9 "$pid" 2>/dev/null || true
echo "✓ $NAME killed"
