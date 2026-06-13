#!/bin/bash

# Cloud (CSMS) 启动脚本
# 用途: 启动 charge-mgt-cloud 服务端，需先确保 PostgreSQL + Kafka 已 up
#       未启动时用 docker compose up -d postgres kafka 启动

set -e

# 设置 CPLUS_INCLUDE_PATH（rdkafka cmake 构建需要，与 gateway 一致）
export CPLUS_INCLUDE_PATH="/Library/Developer/CommandLineTools/SDKs/MacOSX15.5.sdk/usr/include/c++/v1:/Library/Developer/CommandLineTools/SDKs/MacOSX15.5.sdk/usr/include"

# 进入项目根目录（脚本所在目录）
cd "$(dirname "$0")"

# 检查依赖容器
if ! docker ps --format '{{.Names}}' 2>/dev/null | grep -q "^charge-mgt-postgres$"; then
    echo "⚠️  PostgreSQL 容器未运行，尝试启动..."
    docker compose up -d postgres kafka
    echo "   等待 PostgreSQL 就绪..."
    for i in $(seq 1 30); do
        if docker inspect charge-mgt-postgres --format '{{.State.Health.Status}}' 2>/dev/null | grep -q "healthy"; then
            echo "   ✓ PostgreSQL healthy"
            break
        fi
        sleep 1
    done
fi

if ! docker ps --format '{{.Names}}' 2>/dev/null | grep -q "^kafka$"; then
    echo "⚠️  Kafka 容器未运行，尝试启动..."
    docker compose up -d kafka
    sleep 2
fi

# Cloud 配置文件（默认值在 main.rs parse_config_path 中也有兜底）
export CLOUD_CONFIG="crates/charge-mgt-cloud/config/default.yaml"

echo "Starting charge-mgt-cloud..."
echo "   config: $CLOUD_CONFIG"
echo "   database: $(grep 'url:' $CLOUD_CONFIG | head -1 | awk '{print $2}')"
echo "   kafka brokers: $(grep 'brokers:' $CLOUD_CONFIG | head -1 | awk '{print $2}')"
echo ""

# 运行 cloud
cargo run -p charge_mgt_cloud
