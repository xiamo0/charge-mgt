#!/bin/bash

# 关闭 charge-mgt-simulator 服务（所有正在运行的 simulator 实例）

set -e
cd "$(dirname "$0")"

BINARY="target/debug/charge_mgt_simulator"
NAME="charge-mgt-simulator"

pids=$(pgrep -f "$BINARY" 2>/dev/null || true)
if [ -z "$pids" ]; then
    echo "$NAME: not running"
    exit 0
fi

echo "Stopping $NAME (${#pids[@]} instance(s))..."
for pid in $pids; do
    echo "  sending SIGTERM to PID $pid"
    kill "$pid" 2>/dev/null || true
done

sleep 2

# 兜底 kill 仍未退出的
remaining=$(pgrep -f "$BINARY" 2>/dev/null || true)
if [ -n "$remaining" ]; then
    for pid in $remaining; do
        echo "  force killing PID $pid"
        kill -9 "$pid" 2>/dev/null || true
    done
fi

echo "✓ $NAME stopped"
