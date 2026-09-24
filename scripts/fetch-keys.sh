#!/usr/bin/env bash
# 将仓库内公开的更新签名密钥复制到指定目录（本地签名打包用）
# 用法：./scripts/fetch-keys.sh <dest_dir>   默认 /tmp/qrscan-keys
# 说明：密钥（qrscan.key / qrscan.key.pub）已随仓库公开，仅本地签名打包时需要；
#       CI 构建由 GitHub Secrets（TAURI_SIGNING_PRIVATE_KEY / PASSWORD）注入，无需本脚本。
#       解开 qrscan.key 的密码需联系仓库维护者获取。
set -euo pipefail
DEST="${1:-/tmp/qrscan-keys}"
SRC="$(cd "$(dirname "$0")/.." && pwd)/keys"

mkdir -p "$DEST"
cp "$SRC/qrscan.key" "$SRC/qrscan.key.pub" "$DEST/"
echo "密钥已就绪: $DEST/qrscan.key（密码联系维护者获取）"