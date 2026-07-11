#!/usr/bin/env bash
# ラムダ本体のコード量比較（ボイラープレート控除）。
#   nix develop --command translator/scripts/codesize.sh [corpus/v1.lam]
# 「何もしない」空コーパスの生成物を引くことで、埋め込みヘルパー・import 等の
# 固定ボイラープレートを控除し、定義＋表明の正味コード量だけを比較する。
# 分離言語の外部ライブラリ(lam1.*)は demo.* を計測対象にすることで元から除外。
set -euo pipefail
corpus="${1:-corpus/v1.lam}"
cd "$(dirname "$0")/.."

rm -rf /tmp/gsz_c /tmp/gsz_0
cargo run --quiet -- gen --out /tmp/gsz_c "$corpus" >/dev/null
cargo run --quiet -- gen --out /tmp/gsz_0 corpus/empty.lam >/dev/null

# 言語非依存の素朴なトークナイザ（識別子 / 数値 / 各記号1文字）
tok() { grep -oE '[A-Za-z_][A-Za-z0-9_]*|[0-9]+|[^[:space:][:alnum:]_]' "$1" 2>/dev/null | wc -l | tr -d ' '; }

printf "%-9s %8s %8s\n" language dbytes dtokens
printf "%-9s %8s %8s\n" --------- ------ -------
for d in /tmp/gsz_c/*/; do
  name=$(basename "$d")
  f1=$(ls /tmp/gsz_c/"$name"/demo.* 2>/dev/null | head -1) || true
  f0=$(ls /tmp/gsz_0/"$name"/demo.* 2>/dev/null | head -1) || true
  [ -z "${f1:-}" ] && continue
  printf "%-9s %8d %8d\n" "$name" \
    "$(( $(wc -c <"$f1") - $(wc -c <"$f0") ))" \
    "$(( $(tok "$f1") - $(tok "$f0") ))"
done | sort -k2 -n
