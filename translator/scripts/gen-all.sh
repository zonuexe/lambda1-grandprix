#!/usr/bin/env bash
# 全コーパス × 全言語の生成コードを、聴衆が目視できるよう demos/<言語>/<コーパス>/ に出力する。
#
#   translator/scripts/gen-all.sh            # 既定: リポジトリ直下の demos/ へ
#   OUT=/path/to/out translator/scripts/gen-all.sh
#
# 各ディレクトリは自己完結: demo.<ext>（＋分離ライブラリ lam1.*）＋ source.lam（元の DSL）。
# gen はコード生成のみ（コンパイル・実行しない）なので言語ツールチェーン（nix）は不要。
set -euo pipefail
cd "$(dirname "$0")/.."                       # translator/
repo_root="$(cd .. && pwd)"
OUT="${OUT:-$repo_root/demos}"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

cargo build --quiet
rm -rf "$OUT"
mkdir -p "$OUT"

corpora=()
for corpus in "$repo_root"/corpus/*.lam; do
  name="$(basename "$corpus" .lam)"
  corpora+=("$name")
  # 全言語分を tmp/<コーパス>/<言語>/ へ生成
  cargo run --quiet -- gen --out "$tmp/$name" "$corpus" >/dev/null
  # demos/<言語>/<コーパス>/ へ再配置（自己完結のため source.lam も同梱）
  for langdir in "$tmp/$name"/*/; do
    [ -d "$langdir" ] || continue
    lang="$(basename "$langdir")"
    dest="$OUT/$lang/$name"
    mkdir -p "$dest"
    cp "$langdir"* "$dest"/ 2>/dev/null || true
    cp "$corpus" "$dest/source.lam"
  done
done

# 言語一覧（生成された順不同のディレクトリから）
langs=()
for d in "$OUT"/*/; do langs+=("$(basename "$d")"); done

# 索引 README（言語 × コーパスの表）
{
  echo "# 生成コード（聴衆の目視用）"
  echo
  echo "\`translator/scripts/gen-all.sh\` が出力する、全コーパス × 全言語の生成物。"
  echo "\`demos/<言語>/<コーパス>/\` に自己完結で配置（demo.<ext> ＋ 分離ライブラリ ＋ source.lam）。"
  echo "聴衆は自分の言語のディレクトリだけを単独で読めます。"
  echo
  printf '| 言語 |'; for c in "${corpora[@]}"; do printf ' %s |' "$c"; done; echo
  printf '| --- |'; for _ in "${corpora[@]}"; do printf ' --- |'; done; echo
  for lang in "${langs[@]}"; do
    printf '| %s |' "$lang"
    for c in "${corpora[@]}"; do
      f=$(ls "$OUT/$lang/$c"/demo.* 2>/dev/null | head -1)
      if [ -n "$f" ]; then printf ' [%s](%s/%s/) |' "$(basename "$f")" "$lang" "$c"; else printf ' — |'; fi
    done
    echo
  done
} > "$OUT/README.md"

echo "done: $OUT ($(find "$OUT" -type f | wc -l | tr -d ' ') files)"
