//! Lazy K バックエンド（純粋 SKI コンビネータ計算）。zonu-lazyk crate で実行・検証する。
//!
//! Lazy K には λ抽象も名前付き定義も無い。よって:
//!   1. λ を bracket abstraction で S/K/I へ除去する。
//!   2. 各定義は abstraction 後に閉じたコンビネータ項になるので、参照箇所へインライン展開する。
//!
//! 検証は各 assert を「入力を無視して出力バイト列を生成する自己完結プログラム」へ変換し、
//! zonu-lazyk で実行して観測する:
//!   decodeInt  … church 数 N を「N バイト出力」に落とし、出力長 == 期待整数 で判定。
//!   decodeBool … church 真偽値で "true"/"false" のバイト列を選ばせ、出力 == 期待文字列。
//!   decodeJson … Lazy K に現実的な対応物が無いため非対応（skip）。

use super::Backend;
use crate::ast::{Program, Term};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::Path;

/// assert の判定種別。
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Int,
    Bool,
    Unsupported,
}

/// generate 時に構築し exec 時に実行・突合する 1 assert 分。
struct Entry {
    idx: usize,
    kind: Kind,
    program: String,
    expected: String,
}

pub struct LazyK {
    // generate() が構築し、同一インスタンスの exec() が消費する（run_backend は同じ be を使う）。
    entries: RefCell<Vec<Entry>>,
}

impl LazyK {
    pub fn new() -> Self {
        LazyK {
            entries: RefCell::new(Vec::new()),
        }
    }
}

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

fn vcomb(s: &str) -> Comb {
    Comb::Var(s.to_string())
}

/// cons = λh.λt.λf. f h t（Lazy K の出力リスト規約: `list K`=head, `list (K I)`=tail）。
fn cons_comb() -> Comb {
    let body = app(app(vcomb("f"), vcomb("h")), vcomb("t"));
    abstr("h", &abstr("t", &abstr("f", &body)))
}

/// 出力リストの終端: head が 256 以上（EOF）になるセル。
fn end_comb() -> Comb {
    app(app(cons_comb(), church(256)), Comb::I)
}

/// バイト列 s を Lazy K 出力リスト（cons 連鎖＋終端）へ。
fn str_list(s: &str) -> Comb {
    let mut lst = end_comb();
    for &b in s.as_bytes().iter().rev() {
        lst = app(app(cons_comb(), church(b as i64)), lst);
    }
    lst
}

/// decodeInt 用: church 数 term を「値ぶんのバイトを出力する」プログラムへ（入力は無視）。
/// term (cons 'A') end = 'A' を N 個並べたリスト。出力長で N を観測する。
fn wrap_int(term: Comb) -> Comb {
    let cons_b = app(cons_comb(), church(65)); // 'A'（値は長さのみ観測するので何でもよい）
    let list = app(app(term, cons_b), end_comb());
    app(Comb::K, list) // 入力を無視
}

/// decodeBool 用: church 真偽値 term に "true"/"false" を選ばせるプログラム。
fn wrap_bool(term: Comb) -> Comb {
    let sel = app(app(term, str_list("true")), str_list("false"));
    app(Comb::K, sel) // 入力を無視
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
            // decode 系はホスト境界。ラッパを外し内側の項を返す（ネスト時のフォールバック）。
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
        for d in &prog.defs {
            let c = to_comb(&d.term, &map);
            map.insert(d.name.clone(), c);
        }

        let mut out = String::new();
        out.push_str("# λ-1 Lazy K 実行検証プログラム（zonu-lazyk crate で実行）\n");
        out.push_str("# 各 assert は入力を無視して出力バイト列を生成する自己完結プログラム:\n");
        out.push_str("#   int  行 … 出力バイト数 = 期待整数 / bool 行 … 出力バイト列 = 期待文字列\n");
        out.push_str("#   skip 行 … Lazy K に対応物が無い（decodeJson 等）\n");
        out.push_str("# λ は bracket abstraction で除去、名前付き定義はインライン展開済み。\n\n");

        let mut entries = self.entries.borrow_mut();
        entries.clear();
        for (i, a) in prog.asserts.iter().enumerate() {
            let expected = match &a.lhs {
                Term::StrLit(s) => s.clone(),
                _ => "?".to_string(),
            };
            let idx = i + 1;
            let built = match &a.rhs {
                Term::HostCall(name, args) if name == "decodeInt" && !args.is_empty() => {
                    Some((Kind::Int, render(&wrap_int(to_comb(&args[0], &map)))))
                }
                Term::HostCall(name, args) if name == "decodeBool" && !args.is_empty() => {
                    Some((Kind::Bool, render(&wrap_bool(to_comb(&args[0], &map)))))
                }
                _ => None,
            };
            match built {
                Some((kind, program)) => {
                    let ks = if kind == Kind::Int { "int" } else { "bool" };
                    out.push_str(&format!("#@ {} {} {}\n{}\n\n", idx, ks, expected, program));
                    entries.push(Entry { idx, kind, program, expected });
                }
                None => {
                    out.push_str(&format!("#@ {} skip {}\n\n", idx, expected));
                    entries.push(Entry {
                        idx,
                        kind: Kind::Unsupported,
                        program: String::new(),
                        expected,
                    });
                }
            }
        }
        out
    }

    // zonu-lazyk で各 assert を実行し、出力を観測して突合する。成功＝全 assert 緑。
    fn exec(&self, _dir: &Path, _file: &Path) -> std::io::Result<bool> {
        let entries = self.entries.borrow();
        let mut failures = 0usize;
        let mut skipped = 0usize;
        for e in entries.iter() {
            if e.kind == Kind::Unsupported {
                skipped += 1;
                println!("skip assert {} (Lazy K 非対応: decodeJson 等)", e.idx);
                continue;
            }
            let mut buf: Vec<u8> = Vec::new();
            match zonu_lazyk::run(&e.program, std::io::empty(), &mut buf) {
                Ok(()) => {}
                Err(err) => {
                    failures += 1;
                    println!("FAIL assert {}: 実行エラー {:?}", e.idx, err);
                    continue;
                }
            }
            let got = match e.kind {
                Kind::Int => buf.len().to_string(),
                Kind::Bool => String::from_utf8_lossy(&buf).into_owned(),
                Kind::Unsupported => unreachable!(),
            };
            if got == e.expected {
                println!("ok   assert {}", e.idx);
            } else {
                failures += 1;
                println!("FAIL assert {}: {:?} != {:?}", e.idx, got, e.expected);
            }
        }
        if failures > 0 {
            println!("{} failure(s)", failures);
            Ok(false)
        } else {
            if skipped > 0 {
                println!("all green ({} skipped)", skipped);
            } else {
                println!("all green");
            }
            Ok(true)
        }
    }

    fn run_argv(&self, _dir: &Path, _file: &Path) -> Vec<String> {
        vec!["true".to_string()] // exec を override しているため未使用
    }
}
