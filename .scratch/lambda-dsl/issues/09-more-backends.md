# 09: v2 追加バックエンド

Status: ready-for-agent

依存: [02](02-codegen-core.md)

## 内容

v1 の5言語に続き、残りの対象言語のバックエンドを追加する。構造カテゴリごとに注意点が異なる。

## 対象と構造メモ

動的（万能型不要・素のクロージャ）:
- [x] Python, Racket（v1 済み）
- [x] Ruby — `->(x){}` / `f.(x)`
- [x] PHP — arrow fn `fn($x)=>` / 変数は `$_` prefix
- [x] Clojure — JVM Lisp-1 `(fn [x] )` / `(f x)`
- [x] Emacs Lisp — **Lisp-2**。λ適用は `(funcall f x)`、ホスト呼び出しは `(name args)`（関数位置）。`lexical-binding: t` 必須。`t` を変数名に使わない
- [x] Perl — `sub{ my($x)=@_; }` / `$f->($x)`（変数は `$_` prefix）

型付き（タグ付き union の万能型が必要）:
- [x] Haskell, Java, Rust（v1 済み）
- [x] Go — `type D func(D) D` は反射型のみで**観測不能**なため interface＋Fun/Num/Str で union 化
- [x] Standard ML (MLton) — `datatype D = Fun of D->D | Num of int | Str of string`。※SML の識別子は文字始まり必須（`_x` 不正）→ マングリングは `v_` 前置
- [x] Scala — JVM。sealed trait で union（`scala file.scala`）
- [x] Kotlin — JVM。sealed class で union（`kotlinc -include-runtime` → `java -jar`。コンパイル遅め）
- [x] C++（システム clang++）— `std::variant<Fn,long,string>`。capture list からグローバル定義名を除外。※nix の SDK env は不使用でも可
- [x] Swift（システム）— enum union。※nix devShell の `SDKROOT`/`DEVELOPER_DIR` を除去して実行
- [x] Free Pascal — **FPC 3.2.2 は匿名関数非対応** → クロージャ変換（λ→クラス＋Apply）で実装（[10](10-pascal-closure-conversion.md)）。macOS リンカ非互換は `-k-ld_classic` で回避

## 受け入れ条件

- 各言語で v1 コーパスが緑（`translator run --lang <name>`）
- `translator run`（全言語）でまとめて緑
