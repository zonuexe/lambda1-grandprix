# 09: v2 追加バックエンド

Status: ready-for-agent

依存: [02](02-codegen-core.md)

## 内容

v1 の5言語に続き、残りの対象言語のバックエンドを追加する。構造カテゴリごとに注意点が異なる。

## 対象と構造メモ

動的（万能型不要・素のクロージャ）:
- [x] Python, Racket（v1 済み）
- [ ] Ruby — `->(x){}` / `f.(x)`
- [ ] PHP — arrow fn `fn($x)=>` / 変数は `$_` prefix
- [ ] Clojure — JVM Lisp-1 `(fn [x] )` / `(f x)`
- [ ] Emacs Lisp — **Lisp-2**。λ適用は `(funcall f x)`、ホスト呼び出しは `(name args)`（関数位置）。`lexical-binding: t` 必須。`t` を変数名に使わない
- [ ] Perl — `sub{ my($x)=@_; }` / `$f->($x)`

型付き（タグ付き union の万能型が必要）:
- [x] Haskell, Java, Rust（v1 済み）
- [ ] Go — `type D func(D) D` は反射型のみで**観測不能**。struct＋タグ or interface で union 化が必要
- [ ] Scala, Kotlin — JVM。sealed trait / sealed class で union
- [ ] Standard ML (MLton) — `datatype D = Fun of D->D | Num of int | Str of string`
- [ ] Free Pascal — 文構造・関数型。最難の外枠
- [ ] C++（システム clang）— `std::function` ＋ variant/継承で union
- [ ] Swift（システム）— enum で union

## 受け入れ条件

- 各言語で v1 コーパスが緑（`translator run --lang <name>`）
- `translator run`（全言語）でまとめて緑
