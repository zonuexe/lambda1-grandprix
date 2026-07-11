# 10: Free Pascal バックエンド（クロージャ変換）

Status: ready-for-human

依存: [02](02-codegen-core.md)

## 背景

他の17言語はネイティブのクロージャ（無名関数）へ直接翻訳できるが、**Free Pascal 3.2.2 は匿名関数・関数参照（`reference to`）に非対応**（これらは FPC 3.3.1/trunk で導入）。そのため他バックエンドと同じ「λ→ネイティブ無名関数」戦略が使えない。

### 経験的確認（fpc 3.2.2 / flake）

- `{$mode delphi}` の `reference to function` → `Error: Identifier not found "reference"`
- `{$modeswitch functionreferences}` / `{$modeswitch anonymousfunctions}` → `Warning: Illegal compiler switch`（スイッチ自体が存在しない）
- 入れ子手続き（`is nested`）はスタックフレームを掴むため、関数を返して escape させると無効 → 真のクロージャにならない

## 方針の選択肢

1. **クロージャ変換（closure conversion / defunctionalization）**
   - 各 λ を、自由変数を保持するクラス（またはレコード＋関数ポインタ）に変換する
   - 万能型 `TValue`（`ceFun`/`ceNum`/`ceStr` のタグ付き）を用意し、`Fun` は `TClosure`（`Apply` メソッドを持つ基底クラス）を保持
   - λ ごとに `TClosureN = class(TClosure)` を生成し、フィールド＝自由変数、`Apply` に本体を書く
   - `render` を「λ を収集してクラス定義を出力 → 本体は `TClosureN.Create(捕捉値...)` に置換」する専用実装に（Rust/C++ の render override をさらに拡張）
2. **FPC を 3.3.1+ に上げる**（flake の nixpkgs で新しい fpc が入るか要確認）。入るなら他言語と同じ無名関数戦略で済む。

## 推奨

まず選択肢2（新しい FPC が nixpkgs にあるか）を確認し、無ければ選択肢1のクロージャ変換を実装する。クロージャ変換は「無名関数を持たない言語でどうラムダを表現するか」というトーク的にも面白い題材になる。

## 受け入れ条件

- v1 コーパスが Free Pascal で緑（`nix develop` 下の `fpc`）
