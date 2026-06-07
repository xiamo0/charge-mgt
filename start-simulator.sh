#!/bin/bash

# charge-mgt-simulator OCPP 1.6 REPL 启动脚本
# 用法: ./start-simulator.sh [选项]
#     所有未识别的选项会原样透传给 charge_mgt_simulator 二进制。

set -e
cd "$(dirname "$0")"

GATEWAY_URL="${CHARGE_MGT_SIM_GATEWAY:-ws://127.0.0.1:9000}"
CP_ID="${CHARGE_MGT_SIM_ID:-SIM-001}"
NO_COLOR_FLAG=""

print_usage() {
    cat <<EOF
$(basename "$0") —— charge_mgt_simulator REPL 启动器

用法:
    $(basename "$0") [--gateway URL] [--id ID] [--no-color] [--] [其他参数...]

选项:
    -g, --gateway <URL>   Gateway WebSocket 基础 URL (默认: \$CHARGE_MGT_SIM_GATEWAY
                          或 ws://127.0.0.1:9000)
        --id <ID>         charge_point_id (默认: \$CHARGE_MGT_SIM_ID 或 SIM-001)
        --no-color        禁用 ANSI 颜色
    -h, --help            显示本帮助

环境变量:
    CHARGE_MGT_SIM_GATEWAY   默认 gateway URL
    CHARGE_MGT_SIM_ID        默认 charge_point_id

示例:
    $(basename "$0")
    $(basename "$0") --id CP-042
    $(basename "$0") -g ws://192.168.1.100:9000 --id CP-A --no-color
EOF
}

PASSTHRU_ARGS=()
SHOW_HELP=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        -g|--gateway)
            GATEWAY_URL="$2"
            shift 2
            ;;
        --id)
            CP_ID="$2"
            shift 2
            ;;
        --no-color)
            NO_COLOR_FLAG="--no-color"
            shift
            ;;
        -h|--help)
            SHOW_HELP=1
            shift
            ;;
        --)
            shift
            PASSTHRU_ARGS+=("$@")
            break
            ;;
        *)
            PASSTHRU_ARGS+=("$1")
            shift
            ;;
    esac
done

if [[ "$SHOW_HELP" == "1" ]]; then
    print_usage
    exit 0
fi

BINARY_PATH="target/debug/charge_mgt_simulator"
if [ ! -x "$BINARY_PATH" ]; then
    echo "🔨 Building charge_mgt_simulator (first run)..."
    cargo build -p charge_mgt_simulator
fi

echo "🔌 charge-mgt-simulator REPL"
echo "   gateway: $GATEWAY_URL"
echo "   id:      $CP_ID"
echo ""

exec "$BINARY_PATH" --gateway "$GATEWAY_URL" --id "$CP_ID" $NO_COLOR_FLAG "${PASSTHRU_ARGS[@]}"
