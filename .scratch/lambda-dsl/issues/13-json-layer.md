# 13: JSON 層（型付き値）の実装

Status: ready-for-agent

依存: [02](02-codegen-core.md)、仕様 [ADR-0005](../../docs/adr/0005-typed-values-and-json.md)

## 内容

タグ付き値（`pair tag payload`）と汎用 `decodeJson`（再帰で JSON 文字列を構築）を各言語のプレリュードに追加する。raw 層（`encodeInt`/`decodeInt`）と併存。

## プレリュードに足すもの

- 内部: `mkpair`/`fstp`/`sndp`、`church_to_int`、`bool_to_host`、Scott `nil`/`cons`/`is_nil`/`head`/`tail`/`walk`、`c_true`/`c_false`、`sel_k`/`sel_ki`
- 構築子（host 呼び出し）: `jInt` `jBool` `jStr` `jArr` `jObj` `jNull`
- `decodeJson`（tag 分岐 → JSON 文字列）
- 併せて **`emit_str` を `\` と `"` のエスケープに修正**（JSON 期待値が `"` を含むため、二重引用符系言語で必須）

## コーパス

`corpus/json.lam`（int/bool/string/array/object/null ＋ ネスト）。

## 進捗

- [x] Python, Racket（動的）
- [x] Haskell, Java, Rust（型付き union）— v1 スペクトラム完了
- [ ] Ruby, PHP, Perl, Clojure, Emacs（動的・残り）
- [ ] Go, SML, Scala, Kotlin, C++, Swift（型付き・残り）
- [ ] Pascal（クロージャ変換版で JSON 層）
- [ ] Lazy K は対象外（ホスト境界が無い）

## 注意

- 文字列は当面 ASCII/UTF-8 バイトで一致（Haskell はコードポイント基準＝ASCII では一致）。厳密なバイト単位化は詰める余地。
- `emit_str` エスケープは全二重引用符系言語で必要（残り言語の実装時に各々修正）。

## 受け入れ条件

- `translator run --lang <L> corpus/json.lam` が対象言語で緑。raw 層（v1）も緑のまま。
