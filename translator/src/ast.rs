//! DSL の抽象構文木。設計は docs/adr/0004-lambda-dsl-and-translator.md を参照。

#[derive(Debug, Clone)]
pub enum Term {
    /// 変数参照
    Var(String),
    /// λ抽象（単引数。`λx y.M` は入れ子に脱糖済み）
    Lam(String, Box<Term>),
    /// λ適用（並置・左結合）
    App(Box<Term>, Box<Term>),
    /// ホスト境界呼び出し `name[args...]`（プレリュード関数への native 呼び出し）
    HostCall(String, Vec<Term>),
    /// host 整数リテラル（`encode*[…]` の引数位置にのみ現れる）
    IntLit(i64),
    /// host 文字列リテラル（`'...'`）
    StrLit(String),
}

#[derive(Debug, Clone)]
pub struct Def {
    pub name: String,
    pub term: Term,
}

#[derive(Debug, Clone)]
pub struct Assert {
    pub lhs: Term,
    pub rhs: Term,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub defs: Vec<Def>,
    pub asserts: Vec<Assert>,
}
