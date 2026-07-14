# corpus/

DSL ソース（`*.lam`）を置く場所です。各ファイルが比較の題材（コーパス）で、全言語のデモの単一の出処になります。名前付き定義（`I = λx.x`）と `assert`、境界ヘルパー呼び出し `name[…]` からなります（文法は [ADR-0004](../docs/adr/0004-lambda-dsl-and-translator.md)）。

ここを編集したら `translator/scripts/gen-all.sh` で `demos/` を再生成してコミットします。コーパス追加の手順は [`../CONTRIBUTING.md`](../CONTRIBUTING.md) を参照してください。
