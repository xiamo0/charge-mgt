#!/bin/bash

# 一次性关闭所有 charge-mgt 组件
# 用法:
#   ./stop-all.sh              # 只关闭 Rust 进程（cloud/gateway/simulator）
#   ./stop-all.sh --containers # 同时停止 PostgreSQL / Kafka / kafka-ui 容器
#   ./stop-all.sh --all        # 同 --containers

set -e
cd "$(dirname "$0")"

SHUTDOWN_CONTAINERS=false
if [ "$1" = "--containers" ] || [ "$1" = "--all" ]; then
    SHUTDOWN_CONTAINERS=true
fi

stop_by_binary() {
    local binary="target/debug/$1"
    local name="$2"
    local pid
    pid=$(pgrep -f "$binary" 2>/dev/null | head -1)
    if [ -z "$pid" ]; then
        echo "[$name] not running"
        return 0
    fi
    echo "[$name] stopping PID $pid..."
    kill "$pid" 2>/dev/null || true
    for _ in $(seq 1 5); do
        if ! kill -0 "$pid" 2>/dev/null; then
            echo "[$name] ✓ stopped"
            return 0
        fi
        sleep 1
    done
    echo "[$name] ! force killing..."
    kill -9 "$pid" 2>/dev/null || true
    echo "[$name] ✓ killed"
}

stop_by_binary charge_mgt_simulator     simulator
stop_by_binary charge_mgt_gateway       gateway
stop_by_binary charge_mgt_cloud         cloud

if [ "$SHUTDOWN_CONTAINERS" = true ]; then
    echo ""
    echo "--- Stopping docker containers ---"
    docker compose down 2>&1 | tail -10
    echo "✓ containers stopped"
fi

echo ""
echo "Summary:"
echo "  Rust processes:"
pgrep -fl "charge_mgt" 2>/dev/null && echo "    (still running!)" || echo "    none running"
echo "  Docker containers (from compose):"
docker compose ps 2>&1 | grep -v "^NAME" | head -5 || echo "    none running"
