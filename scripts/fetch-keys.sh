#!/usr/bin/env bash
# 从内网 Gitea 密钥仓库拉取更新签名密钥到临时目录（不入库）
# 用法：./scripts/fetch-keys.sh <dest_dir>   默认 /tmp/qrscan-keys
# 凭据来源：QRSCAN_GITEA_URL（完整 URL，可含凭据）或 QRSCAN_GITEA_TOKEN，
# 未设置时从工程仓库 gitea remote 提取内嵌凭据。
set -euo pipefail
DEST="${1:-/tmp/qrscan-keys}"
KEYS_REPO="http://localhost:3002/admin/qrscan-keys.git"

if [ -f "$DEST/qrscan.key" ]; then
  echo "密钥已存在: $DEST/qrscan.key"
  exit 0
fi

URL="${QRSCAN_GITEA_URL:-}"
if [ -z "$URL" ]; then
  TOKEN="${QRSCAN_GITEA_TOKEN:-}"
  if [ -z "$TOKEN" ]; then
    REMOTE="$(git -C "$(dirname "$0")/.." remote get-url gitea 2>/dev/null || true)"
    TOKEN="$(echo "$REMOTE" | sed -E 's#^[a-z]+://([^@]*)@.*#\1#')"
    [ "$TOKEN" = "$REMOTE" ] && TOKEN=""
  fi
  if [ -n "$TOKEN" ]; then
    URL="http://${TOKEN}@localhost:3002/admin/qrscan-keys.git"
  else
    URL="$KEYS_REPO"
  fi
fi

mkdir -p "$DEST"
if ! git clone --depth 1 "$URL" "$DEST" 2>/dev/null; then
  echo "克隆密钥仓库失败（需内网或配置 QRSCAN_GITEA_URL）" >&2
  exit 1
fi
echo "密钥已拉取到: $DEST"
