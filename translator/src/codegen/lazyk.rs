//! Lazy K バックエンド（純粋 SKI コンビネータ計算）。zonu-lazyk crate で実行・検証する。
//!
//! Lazy K には λ抽象も名前付き定義も無い。よって:
//!   1. λ を bracket abstraction で S/K/I へ除去する。
//!   2. 各定義は abstraction 後に閉じたコンビネータ項になるので、参照箇所へインライン展開する。
//!
//! 検証は各 assert の項を zonu-lazyk の `Program::eval_numeral()` で **チャーチ数として直接
//! 観測**する（0.3.0 / ADR-0008。旧: 出力バイト列を数える迂回。ADR-0003 の望ましい性質1）:
//!   decodeInt  … 項（church 数）を eval_numeral → 整数、期待値と比較。
//!   decodeBool … `bool 1 0` を eval_numeral → 1/0 に落とし、"true"/"false" と比較。
//!   decodeJson … 部分対応: jInt/jBool（スカラ）, jStr（文字列）, jNull（"null"）, および
//!                純ラムダで組む typed-int の jArr（フラット int 配列。range など計算リスト
//!                も含む）。string / array は Scott リストを Y コンビネータで Lazy K 出力
//!                リストへ変換し `eval_values` で数値列として観測する。混在配列・object・
//!                ネスト配列は数値列で表せないため skip。

use super::Backend;
use crate::ast::{Program, Term};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::Path;

/// assert の判定種別。
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Int,        // eval_numeral → 整数
    Bool,       // `term 1 0` を eval_numeral → 1/0
    Str,        // eval_values でバイト列 → 文字列復元
    IntArray,   // eval_values で数値列 → [n,...]
    Null,       // 定数 "null"
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

/// decodeBool 用: church 真偽値 term を `term 1 0` に落とす（true→church1 / false→church0）。
/// eval_numeral で 1/0 として観測する。
fn bool_to_numeral(term: Comb) -> Comb {
    app(app(term, church(1)), church(0))
}

fn v(s: &str) -> Comb {
    Comb::Var(s.to_string())
}

// ---- Scott リスト → Lazy K 出力リスト変換（eval_values で読むため） ----
// DSL と同じ Scott 表現: nil=λn.λc.n, cons=λh.λt.λn.λc.c h t
fn scott_nil() -> Comb {
    abstr("n", &abstr("c", &v("n")))
}
fn scott_cons() -> Comb {
    let body = app(app(v("c"), v("h")), v("t"));
    abstr("h", &abstr("t", &abstr("n", &abstr("c", &body))))
}
/// Lazy K の出力リスト cons（`λh.λt.λf. f h t`。`list K`=head, `list (K I)`=tail）。
fn out_pair() -> Comb {
    let body = app(app(v("f"), v("h")), v("t"));
    abstr("h", &abstr("t", &abstr("f", &body)))
}
/// 出力リスト終端: head が church 256（EOF）のセル。
fn out_end() -> Comb {
    app(app(out_pair(), church(256)), Comb::I)
}
/// Y = λf. (λx. f (x x)) (λx. f (x x))（Lazy K は遅延なので発散しない）。
fn y_comb() -> Comb {
    let xx = abstr("x", &app(v("f"), app(v("x"), v("x"))));
    app_lam("f", app(xx.clone(), xx))
}
fn app_lam(x: &str, body: Comb) -> Comb {
    abstr(x, &body)
}
/// 型付き値 → payload を取り出す sndp = λe. e (λa.λb.b)。
fn sndp_sel() -> Comb {
    let ki = abstr("a", &abstr("b", &v("b")));
    app_lam("e", app(v("e"), ki))
}
/// conv = Y (λrec.λlst. lst out_end (λh.λt. out_pair (transform h) (rec t)))。
/// Scott リストを、各要素へ transform を適用した Lazy K 出力リストへ変換する。
fn list_conv(transform: Comb) -> Comb {
    let conscase = {
        let head = app(transform, v("h"));
        let body = app(app(out_pair(), head), app(v("rec"), v("t")));
        abstr("h", &abstr("t", &body))
    };
    let body = abstr("rec", &abstr("lst", &app(app(v("lst"), out_end()), conscase)));
    app(y_comb(), body)
}
/// 入力を無視して「conv payload」の出力リストを返すプログラム。
fn list_program(payload: Comb, transform: Comb) -> Comb {
    app(Comb::K, app(list_conv(transform), payload))
}
/// バイト列 s を Scott リスト（church バイトの cons 連鎖）へ。
fn byte_scott_list(s: &str) -> Comb {
    let mut lst = scott_nil();
    for &b in s.as_bytes().iter().rev() {
        lst = app(app(scott_cons(), church(b as i64)), lst);
    }
    lst
}

/// host 呼び出しを含まない（encodeInt[リテラル] は可）純ラムダ項か。to_comb が panic せずに
/// SKI へ落とせるものだけを true とする。
fn is_pure(t: &Term) -> bool {
    match t {
        Term::Var(_) | Term::IntLit(_) | Term::StrLit(_) => true,
        Term::Lam(_, b) => is_pure(b),
        Term::App(f, x) => is_pure(f) && is_pure(x),
        Term::HostCall(n, args) => n == "encodeInt" && args.iter().all(is_pure),
    }
}

fn json_escape(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

/// コンパイル済みプログラムを種別に応じて観測し、期待値と比較できる文字列にする。
fn observe(p: &zonu_lazyk::Program, kind: Kind) -> Result<String, String> {
    let opts = zonu_lazyk::DecodeOptions::default(); // EOF = church 256
    match kind {
        Kind::Int => p.eval_numeral().map(|n| n.to_string()).map_err(|e| e.to_string()),
        Kind::Bool => p
            .eval_numeral()
            .map(|n| if n != 0 { "true" } else { "false" }.to_string())
            .map_err(|e| e.to_string()),
        Kind::Str => {
            let vals = p.eval_values(&[], &opts).map_err(|e| e.to_string())?;
            let bytes: Vec<u8> = vals.iter().map(|&x| x as u8).collect();
            Ok(json_escape(&String::from_utf8_lossy(&bytes)))
        }
        Kind::IntArray => {
            let vals = p.eval_values(&[], &opts).map_err(|e| e.to_string())?;
            let parts: Vec<String> = vals.iter().map(|x| x.to_string()).collect();
            Ok(format!("[{}]", parts.join(",")))
        }
        Kind::Null | Kind::Unsupported => unreachable!(),
    }
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

/// decodeJson の部分対応。SKI + zonu-lazyk の eval_numeral / eval_values で観測できる形へ。
/// 返すのは (種別, プログラム項)。対応しない形（混在配列・object・ネスト等）は None（skip）。
fn decode_json_case(arg: &Term, map: &BTreeMap<String, Comb>) -> Option<(Kind, Comb)> {
    match arg {
        // jInt[n] … payload は church n。eval_numeral で n を観測。
        Term::HostCall(n, a) if n == "jInt" && a.len() == 1 => match &a[0] {
            Term::IntLit(k) => Some((Kind::Int, church(*k))),
            _ => None,
        },
        // jBool[b] … church 真偽値を `b 1 0` に落として 1/0 を観測。
        Term::HostCall(n, a) if n == "jBool" && a.len() == 1 => {
            Some((Kind::Bool, bool_to_numeral(to_comb(&a[0], map))))
        }
        // jStr['...'] … バイト Scott リストを Lazy K 出力リストへ変換し、eval_values で読む。
        Term::HostCall(n, a) if n == "jStr" && a.len() == 1 => match &a[0] {
            Term::StrLit(s) => Some((Kind::Str, list_program(byte_scott_list(s), Comb::I))),
            _ => None,
        },
        // jNull[] … 定数 "null"。
        Term::HostCall(n, a) if n == "jNull" && a.is_empty() => Some((Kind::Null, Comb::I)),
        // jArr[<pure>] … payload が純ラムダで組む typed-int の Scott リストの時だけ対応。
        // 各要素へ sndp を適用して church int を取り出し、数値列として eval_values で読む。
        // （range など計算リストもここで実際に Lazy K 実行される。混在型 jArr は host 呼び出しを
        //   含み is_pure=false なので skip。）
        Term::HostCall(n, a) if n == "jArr" && a.len() == 1 && is_pure(&a[0]) => {
            Some((Kind::IntArray, list_program(to_comb(&a[0], map), sndp_sel())))
        }
        _ => None,
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
        out.push_str("# λ-1 Lazy K 実行検証プログラム（zonu-lazyk crate で eval_numeral 実行）\n");
        out.push_str("# 各 assert は SKI 項。eval_numeral でチャーチ数として観測する:\n");
        out.push_str("#   int  行 … 項 = 期待整数 / bool 行 … `term 1 0` = 1(true)/0(false)\n");
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
                    Some((Kind::Int, render(&to_comb(&args[0], &map))))
                }
                Term::HostCall(name, args) if name == "decodeBool" && !args.is_empty() => {
                    Some((Kind::Bool, render(&bool_to_numeral(to_comb(&args[0], &map)))))
                }
                // decodeJson: スカラ / 文字列 / null / フラット int 配列（range 含む）を部分対応。
                Term::HostCall(name, args) if name == "decodeJson" && !args.is_empty() => {
                    decode_json_case(&args[0], &map).map(|(k, c)| (k, render(&c)))
                }
                _ => None,
            };
            match built {
                Some((kind, program)) => {
                    let ks = match kind {
                        Kind::Int => "int",
                        Kind::Bool => "bool",
                        Kind::Str => "str",
                        Kind::IntArray => "int[]",
                        Kind::Null => "null",
                        Kind::Unsupported => "skip",
                    };
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
                println!("skip assert {} (Lazy K 非対応: decodeJson の混在配列 / object / ネスト配列)", e.idx);
                continue;
            }
            // Null は定数、それ以外は zonu-lazyk で観測。
            let got = if e.kind == Kind::Null {
                Ok("null".to_string())
            } else {
                zonu_lazyk::Program::compile(&e.program)
                    .map_err(|e| e.to_string())
                    .and_then(|p| observe(&p, e.kind))
            };
            let got = match got {
                Ok(g) => g,
                Err(msg) => {
                    failures += 1;
                    println!("FAIL assert {}: 実行エラー {}", e.idx, msg);
                    continue;
                }
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
