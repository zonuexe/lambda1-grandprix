# 貢献ガイド

このリポジトリへの貢献は主に 2 種類です。用語は [`CONTEXT.md`](CONTEXT.md)、設計背景は [`docs/adr/`](docs/adr/) を参照してください。

- **コーパスを追加する** — 新しい DSL ソース `corpus/<名前>.lam`（比較の題材）を足す。
- **言語を追加する** — 新しいデモ対象言語（＝処理系）のバックエンドを足す。

いずれも作業後は生成物 `demos/` を再生成してコミットします（`demos/` は手編集しません）。開発シェルは `nix develop`（または `direnv allow`）で入ります。

## コーパスを追加する

DSL の文法は既存の `corpus/*.lam` と [ADR-0004](docs/adr/0004-lambda-dsl-and-translator.md) を参照。名前付き定義（`I = λx.x`）と `assert`、境界ヘルパー呼び出し `name[…]` からなります。定義は既出の名前のみ参照でき（前方参照・暗黙再帰なし。再帰は Y/Z コンビネータで明示）、ホスト値は必ず `encode*[…]` で入り `decode*[…]` で出ます。

- [ ] `corpus/<名前>.lam` を作成する。
- [ ] トランスレータで全言語の assert が通ることを確認する。
      ```sh
      cd translator && cargo run -- run ../corpus/<名前>.lam
      ```
      個別言語だけ試すなら `--lang <言語>` を付ける。
- [ ] 生成物を再生成する。
      ```sh
      translator/scripts/gen-all.sh
      ```
- [ ] 変更された `demos/`（`demos/<言語>/<名前>/` 一式と `demos/README.md` の索引）をコミットする。

新しい境界ヘルパー（`encode*`/`decode*`）が必要な場合は、各言語のプレリュード（`languages/<言語>/prelude.<ext>`）に実装を足す必要があります。既存ヘルパーの範囲で書けるコーパスから始めるのが簡単です。

## 言語を追加する

例として言語名を `<lang>` とします（バックエンドの `name()`・`languages/<lang>/`・`demos/<lang>/` はすべてこの名前で揃えます。ハイフンを含む名前も可: 例 `scala-infix`）。

- [ ] **プレリュードを書く**: `languages/<lang>/prelude.<ext>` を作成する。生成コードが依存する境界ヘルパー群（`encode*`/`decode*`、`myassert`、静的型付き言語なら[万能型](CONTEXT.md)の定義）を手書きする。既存言語（`languages/python/prelude.py`, `languages/haskell/prelude.hs` など）を雛形にする。`decode*` は言語間で等価判定を揃えるため**文字列表現**を返す。

- [ ] **バックエンドを実装する**: `translator/src/codegen/<lang>.rs` を作成し、`Backend` trait（`translator/src/codegen/mod.rs` 定義）を実装する。ファイル名はモジュール規約に従い、ハイフンはアンダースコアにする（例: `scala-infix` → `scala_infix.rs`）。最低限、次を実装する:
  - `fn name(&self) -> &'static str` — `<lang>`（`languages/`・`demos/` のディレクトリ名と一致）。
  - `fn ext(&self) -> &'static str` — ソース拡張子。
  - `fn prelude(&self) -> &'static str` — `include_str!("../../../languages/<lang>/prelude.<ext>")`。
  - `emit_lam` / `emit_app` / `emit_host_call` / `emit_str` / `emit_def` / `emit_assert` / `emit_program` — 各構文の描画。多くの規則的処理は trait の `render`/`generate` 既定実装が担うので、言語固有の描画だけを与える。
  - `fn run_argv(&self, dir, file) -> Vec<String>` — 実行コマンド（`argv[0]` が実行ファイル）。
  - 必要に応じて `reserved()`（予約語。`mangle` の既定は予約語に `_` 前置）、`mangle()`（`_` 以外のエスケープや sigil を使う言語）、`build()`（コンパイル段。既定は no-op）、`library()`（ヘルパーを外部ファイルに分離する場合 `(ファイル名, 内容)` を返す。既定は `emit_program` に同梱）を override する。厄介な構造の言語は既定を広く override してよい（[ADR-0004](docs/adr/0004-lambda-dsl-and-translator.md) の T3 ハイブリッド）。

- [ ] **バックエンドを登録する**: `translator/src/codegen/mod.rs` で
  - モジュール宣言 `pub mod <lang>;`（ハイフンはアンダースコア）を追加し、
  - `all_backends()` の `vec![…]` に `Box::new(<lang>::<StructName>),` を追加する。

- [ ] **ツールチェーンを追加する**: 処理系を Nix で管理する場合、`flake.nix` の `devShells.default` の `packages` に該当パッケージを足し、`shellHook` の一覧表示にも追記する。システム付属の処理系（Swift の `/usr/bin/swift` など）を使う場合は追加不要（[ADR-0001](docs/adr/0001-nix-flake-for-toolchains.md) を参照。方針をコメントで残す）。

- [ ] **ビルドと動作確認**:
  ```sh
  cd translator && cargo build
  cargo run -- run --lang <lang> ../corpus/v1.lam   # assert が全て通ることを確認
  ```

- [ ] **生成物を再生成してコミットする**:
  ```sh
  translator/scripts/gen-all.sh
  ```
  新しい `demos/<lang>/<各コーパス>/` と、更新された `demos/README.md` の索引をコミットする。

## コミットの範囲

コーパス追加・言語追加のいずれも、手書きソース（`corpus/`・`languages/`・`translator/`・`flake.nix`）と、それを反映した生成物 `demos/` を**同じ変更に含めて**コミットしてください。`demos/` は `gen-all.sh` の出力なので、手で編集した差分は再生成で失われます。
