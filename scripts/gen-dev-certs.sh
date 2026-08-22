#!/bin/bash
# 生成 OCPP 网关开发用自签 TLS 证书
# 仅用于开发/测试；生产请用 Let's Encrypt 或企业 CA 签名证书
set -euo pipefail

CERT_DIR="crates/charge-mgt-gateway/certs"
mkdir -p "$CERT_DIR"

CN="${1:-gateway.local}"
DAYS="${2:-365}"

if [ -f "$CERT_DIR/server-cert.pem" ] && [ -f "$CERT_DIR/server-key.pem" ]; then
    echo "证书已存在：$CERT_DIR/server-cert.pem"
    echo "如需重新生成，请先删除 certs/ 目录下的文件"
    exit 0
fi

echo "生成自签证书（CN=$CN，有效期 $DAYS 天）..."
openssl req -x509 -newkey rsa:2048 -nodes \
    -keyout "$CERT_DIR/server-key.pem" \
    -out "$CERT_DIR/server-cert.pem" \
    -days "$DAYS" \
    -subj "/CN=$CN"

echo "✓ 证书生成完成："
echo "  证书: $CERT_DIR/server-cert.pem"
echo "  私钥: $CERT_DIR/server-key.pem"
echo ""
echo "下一步：把 certs/server-cert.pem 导入到充电桩的信任列表"
