#!/bin/bash

# Gateway 启动脚本

# 设置 CPLUS_INCLUDE_PATH（rdkafka 编译所需）
export CPLUS_INCLUDE_PATH="/Library/Developer/CommandLineTools/SDKs/MacOSX15.5.sdk/usr/include/c++/v1:/Library/Developer/CommandLineTools/SDKs/MacOSX15.5.sdk/usr/include"

# 进入项目目录
cd "$(dirname "$0")"

export CONFIG_PATH="crates/charge-mgt-gateway/config/default"

echo "Starting charge-mgt-gateway..."

# 运行 gateway
cargo run -p charge_mgt_gateway
