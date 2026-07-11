//! Lazy K バックエンド（純粋 SKI コンビネータ計算・テストデータ生成）。
//!
//! Lazy K には λ抽象も名前付き定義も無い。よって:
//!   1. λ を bracket abstraction で S/K/I へ除去する。
//!   2. 各定義は abstraction 後に閉じたコンビネータ項になるので、参照箇所へインライン展開する
//!      （閉じている＝自由変数ゼロなので変数捕獲は起きない）。
//! 出力は SKIスタイル（S/K/I ＋並置＋括弧）の純粋なコンビネータ項のみ（IO なし）。
//! Lazy K インタプリタは本環境に無いため実行はせず、静的ファイルを生成するだけ。

use super::Backend;
use crate::ast::{Program, Term};
use std::collections::BTreeMap;
use std::path::Path;

pub struct LazyK;

/// SKI コンビネータ項（Var は abstraction の途中でのみ現れる中間表現）。
#[derive(Clone)]
enum Comb {
    S,
    K,
    I,
    Var(String),
    App(Box<Comb>, Box<Comb>),
}

fn app(a: Comb, b: Comb) -> Comb {
    Comb::App(Box::new(a), Box::new(b))
}

fn occurs(x: &str, c: &Comb) -> bool {
    match c {
        Comb::Var(v) => v == x,
        Comb::App(f, g) => occurs(x, f) || occurs(x, g),
        _ => false,
    }
}

/// bracket abstraction: `λx.c` を S/K/I 項へ。
fn abstr(x: &str, c: &Comb) -> Comb {
    match c {
        Comb::Var(v) if v == x => Comb::I,
        _ if !occurs(x, c) => app(Comb::K, c.clone()),
        Comb::App(f, g) => app(app(Comb::S, abstr(x, f)), abstr(x, g)),
        // ここに来るのは Var(x) のみ（上で処理済み）だが網羅のため
        _ => app(Comb::K, c.clone()),
    }
}

/// チャーチ数 N = λf.λx. f^N x のコンビネータ。
fn church(n: i64) -> Comb {
    let mut body = Comb::Var("x".to_string());
    for _ in 0..n {
        body = app(Comb::Var("f".to_string()), body);
    }
    let inner = abstr("x", &body);
    abstr("f", &inner)
}

/// DSL の Term を SKI コンビネータへ。定義参照は map からインライン展開。
fn to_comb(t: &Term, map: &BTreeMap<String, Comb>) -> Comb {
    match t {
        Term::Var(v) => map.get(v).cloned().unwrap_or_else(|| Comb::Var(v.clone())),
        Term::Lam(p, body) => {
            let b = to_comb(body, map);
            abstr(p, &b)
        }
        Term::App(f, x) => app(to_comb(f, map), to_comb(x, map)),
        Term::HostCall(name, args) => match name.as_str() {
            "encodeInt" => match args.first() {
                Some(Term::IntLit(n)) => church(*n),
                _ => panic!("encodeInt expects an int literal"),
            },
            // decode 系はホスト境界。Lazy K に対応物が無いのでラッパを外し内側の項を返す。
            "decodeInt" | "decodeBool" => to_comb(&args[0], map),
            other => panic!("host call `{}` has no Lazy K equivalent", other),
        },
        Term::IntLit(_) => panic!("bare int is not representable in Lazy K"),
        Term::StrLit(_) => panic!("bare string is not representable in Lazy K"),
    }
}

/// SKIスタイルの文字列へ（並置は左結合、引数の App は括弧）。
fn render(c: &Comb) -> String {
    match c {
        Comb::S => "S".to_string(),
        Comb::K => "K".to_string(),
        Comb::I => "I".to_string(),
        Comb::Var(v) => format!("<{}>", v), // 最終項には現れない
        Comb::App(f, x) => format!("{} {}", render(f), atom(x)),
    }
}

fn atom(c: &Comb) -> String {
    match c {
        Comb::App(..) => format!("({})", render(c)),
        _ => render(c),
    }
}

impl Backend for LazyK {
    fn name(&self) -> &'static str {
        "lazyk"
    }
    fn ext(&self) -> &'static str {
        "lazy"
    }
    fn prelude(&self) -> &'static str {
        ""
    }

    // generate を全面 override するため emit_* は未使用（trait 要件のスタブ）。
    fn emit_lam(&self, _p: &str, _b: &str) -> String {
        unreachable!("lazyk overrides generate")
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
        let mut out = String::new();
        out.push_str("# λ-1 Lazy K test data — pure SKI combinator terms (SKI style)\n");
        out.push_str("# 各非コメント行は独立したコンビネータ項（ファイル全体で1プログラムではない）。\n");
        out.push_str("# λ は bracket abstraction で除去、名前付き定義はインライン展開済み。\n\n");

        out.push_str("# --- definitions ---\n");
        for d in &prog.defs {
            let c = to_comb(&d.term, &map);
            out.push_str(&format!("# {}\n{}\n", d.name, render(&c)));
            map.insert(d.name.clone(), c);
        }

        out.push_str("\n# --- assertions (inner computation; expected value in comment) ---\n");
        for (i, a) in prog.asserts.iter().enumerate() {
            let expected = match &a.lhs {
                Term::StrLit(s) => s.clone(),
                _ => "?".to_string(),
            };
            let c = to_comb(&a.rhs, &map);
            out.push_str(&format!("# assert {}: expected {}\n{}\n", i + 1, expected, render(&c)));
        }
        out
    }

    // 実行はしない（Lazy K インタプリタは本環境に無い）。生成物が成果物。
    fn exec(&self, _dir: &Path, file: &Path) -> std::io::Result<bool> {
        println!(
            "generated {} (pure SKI combinator terms; no interpreter to run here)",
            file.display()
        );
        Ok(true)
    }

    fn run_argv(&self, _dir: &Path, _file: &Path) -> Vec<String> {
        vec!["true".to_string()] // exec を override しているため未使用
    }
}
