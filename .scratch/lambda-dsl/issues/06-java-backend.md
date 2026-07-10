# 06: Java バックエンド

Status: ready-for-agent

依存: [05](05-haskell-backend.md)

## 内容

型付き OO・関数型インタフェースの代表として Java バックエンドを実装する。

## 仕様

- 万能型: `interface D { D apply(D x); }`（関数型インタフェース）
- `λx.M` → `(D)(_x -> M)`、適用 `f x` → `f.apply(x)`、定義 → `static final D _name = expr;`（クラスの静的フィールド、初期化順序に注意）
- プレリュード `translator/preludes/java.java`: `D`、`encodeInt`/`decodeInt`/`decodeBool`（String 返し）、`myassert`。`main` で assert
- 名前マングリング＋Java 予約語（`if`, `true`, `false`, `assert` 等）対応

## 受け入れ条件

- v1 コーパスを Java 生成・コンパイル・実行し全 assert 緑（`nix develop` 下の `javac`/`java`）
- 静的初期化順序（前方参照なしの規約と整合）で落ちないこと
