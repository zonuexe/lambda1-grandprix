# 01: クレート骨組み ＋ lexer/parser/AST

Status: ready-for-agent

## 内容

`translator/` に Rust クレート（package `lambda1-translator`）を作り、DSL の字句・構文解析と AST を実装する。

## 仕様

- 字句: 識別子、`λ` / `\`、`.`、`=`、`==`、`(` `)`、`[` `]`、`,`、`---`、整数リテラル、文字列リテラル（`'...'`）、`assert`、`#` 行コメント
- 構文:
  - プログラム = 定義セクション `---` 表明セクション
  - 定義 = `name = expr`
  - 式（ラムダ世界）= 変数 / `λx.M`（`λx y.M` は `λx.λy.M` へ脱糖）/ 適用（並置・左結合）/ `( expr )` / ホスト呼び出し `name[ arg, ... ]`
  - ホスト呼び出しの引数 = host リテラル（int / 文字列）または式
  - 表明 = `assert hostExpr == hostExpr`
- AST: `Term`（`Var`, `Lam`, `App`, `HostCall{name, args}`, `IntLit`, `StrLit`）、`Def{name, term}`、`Assert{lhs, rhs}`、`Program{defs, asserts}`

## 受け入れ条件

- `docs/proposal.md` の例と v1 コーパス断片をパースでき、AST を `Debug` 出力できる
- 単体テスト: 並置の左結合（`a b c` = `((a b) c)`）、多引数糖衣、`name[…]` と適用の区別、コメント/セクション区切り
