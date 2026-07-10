# 05: Haskell バックエンド

Status: ready-for-agent

依存: [03](03-python-backend-runner.md)

## 内容

ML 系・遅延評価の代表として Haskell バックエンドを実装する。**万能型を初めて要する**バックエンド。

## 仕様

- 万能型: `newtype D = D (D -> D)`、`app :: D -> D -> D; app (D f) x = f x`
- `λx.M` → `D (\_x -> M)`、適用 `f x` → `app f x`、定義 → `_name = expr`（トップレベル束縛）
- プレリュード `translator/preludes/haskell.hs`: 万能型 `D`/`app`、`encodeInt :: Int -> D`、`decodeInt :: D -> String`、`decodeBool :: D -> String`、`myassert`。`main` で assert を評価
- 適用は必ず `app` 経由（`emit_app` を override）

## 受け入れ条件

- v1 コーパスを Haskell 生成・実行し全 assert 緑（`nix develop` 下の `runghc`/`ghc`）
- 遅延評価ゆえの差異（あれば）を issue コメントに記録
