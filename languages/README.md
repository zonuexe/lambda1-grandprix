# languages/

言語ごとの手書きプレリュード（`<言語>/prelude.<ext>`）を置く場所です。プレリュードは生成コードが依存する境界ヘルパー群（`encode*`/`decode*`、`myassert`、静的型付き言語なら[万能型](../CONTEXT.md)の定義）で、[トランスレータ](../translator/)が `include_str!` で取り込み、生成コードの後ろに連結します。ディレクトリ名は対応するバックエンドの `name()`（および `demos/<言語>/`）と一致します。

言語（デモ対象）を追加する手順（プレリュード作成・`translator/src/codegen/<言語>.rs` 実装・`mod.rs` 登録・`flake.nix`・再生成）は [`../CONTRIBUTING.md`](../CONTRIBUTING.md) を参照してください。
