# 01: corpus / preludes をトップレベルへ移動

Status: ready-for-human

## 作業内容

- `git mv translator/corpus corpus`
- `translator/preludes/<lang>.<ext>` → `languages/<lang>/prelude.<ext>` へ git mv で再編
  （例: `preludes/php_ski.php` → `languages/php-ski/prelude.php`。ディレクトリ名は
  code/ 配下の言語ディレクトリ名（ハイフン区切り）に合わせる）
- translator 側の追従:
  - 各 `translator/src/codegen/*.rs` の `include_str!("../../preludes/…")` を新パスへ
  - `translator/scripts/gen-all.sh` の `corpus/*.lam` 参照を `../corpus` 相当へ
  - `translator/src` 内のその他の `corpus` / `preludes` パス参照を grep して全て追従
- `.gitignore` のコメント・パスの整合確認

## 受け入れ条件

- `cargo build`（translator/ で）成功
- `rg -l 'translator/corpus|translator/preludes|preludes/' --hidden -g '!.git'` で旧パス参照ゼロ
  （docs 内の記述は issue 03 で扱うため、コード・スクリプトのみ対象）

## Comments

- `git mv translator/corpus corpus`、`translator/preludes/*` を `languages/<lang>/prelude.<ext>`（22言語、code/ のハイフン区切り名に合わせる）へ git mv 済み。
- 22件の `include_str!` パス（`../../../languages/<lang>/prelude.<ext>`）、`gen-all.sh`/`codesize.sh` の corpus 参照（`$repo_root/corpus/...`）、`.gitignore` コメントを追従。
- `cargo build`（translator/）成功、`rg -l 'translator/corpus|translator/preludes|preludes/' --hidden -g '!.git'` は docs（issue 03 対象）以外ヒットなし。
