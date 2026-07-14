# 02: code/ → demos/ 改名と再生成

Status: ready-for-human

## 作業内容

- `translator/scripts/gen-all.sh` の既定 OUT を `$repo_root/demos` に変更し、
  生成する索引 README の文言（`code/<言語>/<コーパス>/` 等）も追従
- `translator/scripts/codesize.sh` 等、`code/` を参照するスクリプトを追従
- `git rm -r code` 後に `gen-all.sh` を実行して `demos/` を生成（生成物は
  スクリプトが作るので git mv ではなく再生成でよい）
- 再生成前後で内容一致を確認（`diff -r` で旧 code/ と demos/ を比較。索引 README の
  パス文言差分のみ許容）

## 依存

- 01 完了後（gen-all.sh が新 corpus パスで動く必要がある）

## 受け入れ条件

- `demos/` に全言語 × 全コーパスが生成され、旧 `code/` と demo/lam1/source の内容一致
- リポジトリ内のコード・スクリプトに `code/` 参照が残らない（docs は issue 03）

## Comments

- `gen-all.sh` の既定 OUT を `$repo_root/demos` に変更し、索引 README・コメント文言も `demos/` に追従。`main.rs`/`.gitignore` の `code/` 言及も更新（`codesize.sh` は元々 `code/` 未参照）。
- 旧 `code/` をスクラッチパッドへ退避コピー後 `git rm -r code`、`gen-all.sh` 実行で `demos/`（286ファイル）を再生成。
- `diff -r` で退避コピーと `demos/` を比較した結果、差分は索引 README のパス文言（`code/` → `demos/`）1箇所のみで、生成コード本体（demo.*/lam1.*/source.lam）は全言語×全コーパスで完全一致。
