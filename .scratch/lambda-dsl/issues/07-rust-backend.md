# 07: Rust バックエンド

Status: ready-for-agent

依存: [05](05-haskell-backend.md)

## 内容

システム系・最難のクロージャ型付けの代表として Rust バックエンドを実装する（実装言語でもあり早期 derisk 対象）。

## 仕様

- 万能型（タグ付き union）: `#[derive(Clone)] enum D { Fun(Rc<dyn Fn(D) -> D>), Num(i64), Str(String) }`、`impl D { fn app(&self, x: D) -> D { match self { D::Fun(f) => f(x), _ => panic!() } } }`、コンストラクタ `fn lam(f: impl Fn(D)->D + 'static) -> D`
- `λx.M` → `lam(move |_x| M)`（`move` とクローン戦略に注意）、適用 `f x` → `f.app(x)`、定義 → `let _name: D = expr;`（`main` 内 or `OnceCell`）
- プレリュード `translator/preludes/rust.rs`: 万能型 `D`、`encodeInt`/`decodeInt`/`decodeBool`（String 返し）、`myassert`
- 所有権・`Clone`・`'static` 境界の扱いが山場。生成物は `rustc` で単体コンパイル

## 受け入れ条件

- v1 コーパスを Rust 生成・コンパイル・実行し全 assert 緑（`nix develop` 下の `rustc`）
- クローン/所有権で借用エラーが出ないコード生成戦略を確立し issue にメモ
