//! PHP (SKI / point-free) バックエンド — Lazy K 方式を PHP で。
//!
//! DSL の λ を bracket abstraction で S/K/I へ除去し、名前付き定義は参照箇所へインライン
//! 展開する（定義は閉じた項なので変数捕獲は起きない）。生成される項は **S()/K()/I()
//! コンビネータの適用のみで、DSL 定義に対応するホスト変数（`$pair` 等）は一切現れない**。
//! 境界だけはホスト関数: `encodeInt[リテラル]` は church 項へ inline、他の host 呼び出し
//! （decodeInt / jInt / decodeJson …）はプレリュード関数呼び出しとして残す。

use super::Backend;
use crate::ast::{Program, Term};
use std::collections::BTreeMap;
use std::path::Path;

pub struct PhpSki;

/// SKI 項。App / Var は abstraction 用の中間表現。Host / Int / Str は境界（λ の外側にのみ
/// 現れ、abstr の対象にはならない）。
#[derive(Clone)]
enum Comb {
    S,
    K,
    I,
    Var(String),
    App(Box<Comb>, Box<Comb>),
    Host(String, Vec<Comb>),
    Int(i64),
    Str(String),
}

fn app(a: Comb, b: Comb) -> Comb {
    Comb::App(Box::new(a), Box::new(b))
}

fn occurs(x: &str, c: &Comb) -> bool {
    match c {
        Comb::Var(v) => v == x,
        Comb::App(f, g) => occurs(x, f) || occurs(x, g),
        Comb::Host(_, args) => args.iter().any(|a| occurs(x, a)),
        _ => false,
    }
}

/// bracket abstraction: `λx.c` を S/K/I へ。c は純ラムダ由来（Host を含まない）。
fn abstr(x: &str, c: &Comb) -> Comb {
    match c {
        Comb::Var(v) if v == x => Comb::I,
        _ if !occurs(x, c) => app(Comb::K, c.clone()),
        Comb::App(f, g) => app(app(Comb::S, abstr(x, f)), abstr(x, g)),
        Comb::Host(..) => panic!("cannot bracket-abstract over a host call inside a lambda"),
        _ => app(Comb::K, c.clone()),
    }
}

/// チャーチ数 N = λf.λx. f^N x のコンビネータ。
fn church(n: i64) -> Comb {
    let mut body = Comb::Var("x".to_string());
    for _ in 0..n {
        body = app(Comb::Var("f".to_string()), body);
    }
    abstr("f", &abstr("x", &body))
}

/// DSL Term → SKI 項。定義参照は map からインライン展開。
fn to_comb(t: &Term, map: &BTreeMap<String, Comb>) -> Comb {
    match t {
        Term::Var(v) => map.get(v).cloned().unwrap_or_else(|| Comb::Var(v.clone())),
        Term::Lam(p, body) => {
            let b = to_comb(body, map);
            abstr(p, &b)
        }
        Term::App(f, x) => app(to_comb(f, map), to_comb(x, map)),
        Term::HostCall(name, args) => {
            if name == "encodeInt" {
                match args.as_slice() {
                    [Term::IntLit(n)] => church(*n),
                    _ => panic!("encodeInt expects a single int literal"),
                }
            } else {
                Comb::Host(name.clone(), args.iter().map(|a| to_comb(a, map)).collect())
            }
        }
        Term::IntLit(n) => Comb::Int(*n),
        Term::StrLit(s) => Comb::Str(s.clone()),
    }
}

/// SKI 項を PHP 式へ。適用は PHP のクロージャ呼び出し `f(x)`（左結合で連鎖）。
fn render(c: &Comb) -> String {
    match c {
        Comb::S => "S()".to_string(),
        Comb::K => "K()".to_string(),
        Comb::I => "I()".to_string(),
        Comb::App(f, x) => format!("{}({})", render(f), render(x)),
        Comb::Host(name, args) => {
            let a: Vec<String> = args.iter().map(render).collect();
            format!("{}({})", name, a.join(", "))
        }
        Comb::Int(n) => n.to_string(),
        Comb::Str(s) => format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'")),
        Comb::Var(v) => format!("/*unbound:{}*/", v), // 純 SKI では現れない
    }
}

impl Backend for PhpSki {
    fn name(&self) -> &'static str {
        "php-ski"
    }
    fn ext(&self) -> &'static str {
        "php"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/php_ski.php")
    }
    fn library(&self) -> Option<(String, String)> {
        Some(("lam1.php".into(), self.prelude().into()))
    }

    // generate を全面 override するため emit_* は未使用（trait 要件のスタブ）。
    fn emit_lam(&self, _p: &str, _b: &str) -> String {
        unreachable!("php-ski overrides generate (bracket abstraction)")
    }
    fn emit_app(&self, _f: &str, _x: &str) -> String {
        unreachable!()
    }
    fn emit_host_call(&self, _n: &str, _a: &[String]) -> String {
        unreachable!()
    }
    fn emit_str(&self, _s: &str) -> String {
        unreachable!()
    }
    fn emit_def(&self, _n: &str, _t: &str) -> String {
        unreachable!()
    }
    fn emit_assert(&self, _i: usize, _l: &str, _r: &str) -> String {
        unreachable!()
    }
    fn emit_program(&self, _d: &[String], _a: &[String]) -> String {
        unreachable!()
    }

    fn generate(&self, prog: &Program) -> String {
        let mut map: BTreeMap<String, Comb> = BTreeMap::new();
        for d in &prog.defs {
            let c = to_comb(&d.term, &map);
            map.insert(d.name.clone(), c);
        }

        let mut s = String::new();
        s.push_str("<?php\nrequire __DIR__ . '/lam1.php';  // S/K/I ＋境界ヘルパーは lam1.php\n");
        s.push_str("// DSL 定義はすべて S/K/I へインライン展開済み（ホスト変数を持たない）。\n");
        s.push_str("\n// --- assertions ---\n");
        for (i, a) in prog.asserts.iter().enumerate() {
            let l = render(&to_comb(&a.lhs, &map));
            let r = render(&to_comb(&a.rhs, &map));
            s.push_str(&format!("_check({}, {}, 'assert {}');\n", l, r, i + 1));
        }
        s.push_str("\n_finish();\n");
        s
    }

    fn run_argv(&self, _dir: &Path, file: &Path) -> Vec<String> {
        vec!["php".to_string(), file.to_string_lossy().into_owned()]
    }
}
