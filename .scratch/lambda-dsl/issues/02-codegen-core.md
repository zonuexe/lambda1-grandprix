# 02: codegen コア（LanguageSpec / Backend trait / プレリュード / マングリング）

Status: ready-for-agent

依存: [01](01-scaffold-parser.md)

## 内容

AST から各言語ソースを生成する共通基盤（ADR-0004 の T3 ハイブリッド）を実装する。

## 仕様

- `Backend` trait: `emit_var`, `emit_lam(param, body)`, `emit_app(f, x)`, `emit_host_call(name, args)`, `emit_def(name, term)`, `emit_program(defs, asserts)`, `prelude() -> &str`, `reserved_words()`, `file_extension()`
- `LanguageSpec`（データ駆動デフォルト実装）: クロージャの開/閉トークン、適用の描画（native `f(x)` か 万能型 `apply` 経由か）、束縛構文、プログラム外枠。規則的な言語はこれで賄い、難物は trait override
- **マングリング**: 変数・定義名は `_` prefix（予約語衝突回避）。`reserved_words()` を各言語が提供
- **プレリュード**: 各言語の手書きソース（`translator/preludes/<lang>.*`）を埋め込み（`include_str!`）、生成コードの前に連結。`encodeInt`/`decodeInt`/`decodeBool`/`myassert`/万能型 `D` を含む
- ホスト呼び出しは native 関数呼び出しに、λ適用は（型付き言語では）万能型 `apply` 経由に落とす

## 受け入れ条件

- ダミー1言語で「定義＋assert」の最小プログラムを文字列生成できる
- 予約語（`if`, `true` 等）が `_if`, `_true` にマングリングされる単体テスト
