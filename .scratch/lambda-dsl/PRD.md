# PRD: ラムダ計算 DSL ＋ 各言語トランスレータ（v1）

## 背景

`docs/proposal.md` のトーク「λ-1グランプリ」で、各言語のラムダ式を比較する[[CONTEXT|デモ]]を単一の出処から生成するための DSL とトランスレータ。設計は [ADR-0004](../../docs/adr/0004-lambda-dsl-and-translator.md)、用語は [CONTEXT.md](../../CONTEXT.md) を参照。

## ゴール

型なしラムダ計算の DSL で書いた定義と表明を、各言語のネイティブなクロージャへ**直接翻訳**し、生成コードを実行して `assert` が緑になることを確認できる状態にする。

## v1 スコープ

- **対象言語（5言語スペクトラム）**: Python（動的）/ Racket（Lisp-1）/ Haskell（ML系・遅延）/ Java（関数型インタフェース）/ Rust（Rc<dyn Fn>）
- **題材（コーパス）**: SKI（I, K, S）＋ チャーチ数（`encodeInt`/`decodeInt`, `zero`, `succ`, `add`）＋ チャーチ真偽値/if（`true`, `false`, `if`, `and`, `not`, `decodeBool`）
- **実装**: Rust 単一クレート（`translator/`）。lexer/parser/AST →`LanguageSpec`/`Backend` trait → 5言語バックエンド＋手書きプレリュード → コーパス実行ランナー

## 非ゴール（v1 では作らない）

- SKI 中間表現 / bracket abstraction（ADR-0004 で不採用）
- `decodeList`/`decodeAssoc`/`Ascii` 等の族（子要素表現が未解決のため保留。不要の可能性）
- Y コンビネータ/階乗による strict↔lazy 比較（v2）
- Emacs Lisp（Lisp-2）/ Go / Free Pascal / その他 残り言語のバックエンド（v2 以降）
- Lazy K インタプリタ（別 issue。同一クレートに後日統合）

## DSL 文法（要約）

- `λx.M`（`\x.M` 可、`λx y.M` は糖衣）/ 適用＝並置左結合 / グルーピング `( )`
- ホスト境界 `name[…]`（プレリュード関数への native 呼び出し）/ host 値は `encode*[…]` で入り `decode*[…]`（文字列を返す）で出る
- 定義 `name = expr`（既出参照のみ・暗黙再帰なし）/ `assert a == b`（文字列等価）
- コメント `#` / `---` で定義・表明セクションを区切る

## マイルストーン（issues）

1. [01](issues/01-scaffold-parser.md) クレート骨組み ＋ lexer/parser/AST
2. [02](issues/02-codegen-core.md) codegen コア（LanguageSpec / Backend trait / プレリュード / マングリング）
3. [03](issues/03-python-backend-runner.md) Python バックエンド ＋ 実行ランナー
4. [04](issues/04-racket-backend.md) Racket バックエンド
5. [05](issues/05-haskell-backend.md) Haskell バックエンド
6. [06](issues/06-java-backend.md) Java バックエンド
7. [07](issues/07-rust-backend.md) Rust バックエンド
8. [08](issues/08-v1-corpus.md) v1 コーパス ＋ 全5言語で assert 緑

## 受け入れ条件

`nix develop` 下で `translator` にコーパスを与えると、5言語すべてのコードを生成・実行し、全 `assert` が緑になる。
