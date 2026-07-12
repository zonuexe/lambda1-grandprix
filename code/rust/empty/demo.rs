// λ-1 translator — Rust prelude
#![allow(non_snake_case, unused_variables, unused_mut, dead_code)]
use std::rc::Rc;

// 万能型（タグ付き union）: ラムダ世界は Fun のみ、Num/Str は境界のみ。
#[derive(Clone)]
enum D {
    Fun(Rc<dyn Fn(D) -> D>),
    Num(i64),
    Str(String),
}

impl D {
    fn app(&self, x: D) -> D {
        match self {
            D::Fun(f) => f(x),
            _ => panic!("applied a non-function"),
        }
    }
}

fn lam<F: Fn(D) -> D + 'static>(f: F) -> D {
    D::Fun(Rc::new(f))
}

fn encodeInt(n: i64) -> D {
    lam(move |f| {
        lam(move |x| {
            let mut acc = x;
            for _ in 0..n {
                acc = f.app(acc);
            }
            acc
        })
    })
}

fn decodeInt(t: D) -> String {
    let incr = lam(|v| match v {
        D::Num(k) => D::Num(k + 1),
        _ => panic!("incr: not a number"),
    });
    match t.app(incr).app(D::Num(0)) {
        D::Num(k) => k.to_string(),
        _ => panic!("decodeInt: not a number"),
    }
}

fn decodeBool(t: D) -> String {
    match t
        .app(D::Str("true".to_string()))
        .app(D::Str("false".to_string()))
    {
        D::Str(s) => s,
        _ => panic!("decodeBool: not a bool"),
    }
}

fn check(label: &str, a: String, b: String) -> i32 {
    if a == b {
        println!("ok   {}", label);
        0
    } else {
        println!("FAIL {}: {:?} != {:?}", label, a, b);
        1
    }
}

fn main() {
    let mut _failures = 0;


    if _failures > 0 {
        println!("{} failure(s)", _failures);
        std::process::exit(1);
    }
    println!("all green");
}

// ============ JSON 層（型付き値。ADR-0005） ============
fn sel_k() -> D { lam(|a| lam(move |_b| a.clone())) }   // λa.λb.a
fn sel_ki() -> D { lam(|_a| lam(|b| b)) }               // λa.λb.b
fn c_true() -> D { lam(|t| lam(move |_f| t.clone())) }
fn c_false() -> D { lam(|_t| lam(|f| f)) }

fn mkpair(a: D, b: D) -> D {
    lam(move |s: D| s.app(a.clone()).app(b.clone()))
}
fn fstp(p: &D) -> D { p.app(sel_k()) }
fn sndp(p: &D) -> D { p.app(sel_ki()) }

fn church_to_int(c: &D) -> i64 {
    let incr = lam(|v| match v { D::Num(k) => D::Num(k + 1), _ => panic!("incr") });
    match c.app(incr).app(D::Num(0)) {
        D::Num(k) => k,
        _ => panic!("church_to_int"),
    }
}
fn bool_to_host(c: &D) -> bool {
    match c.app(D::Str("T".to_string())).app(D::Str("F".to_string())) {
        D::Str(s) => s == "T",
        _ => false,
    }
}

fn nil_h() -> D { lam(|n| lam(move |_c| n.clone())) }
fn cons_h(h: D, t: D) -> D {
    lam(move |_n| {
        let h = h.clone();
        let t = t.clone();
        lam(move |c: D| c.app(h.clone()).app(t.clone()))
    })
}
fn is_nil(lst: &D) -> bool {
    let cons_case = lam(|_h| lam(|_t| c_false()));
    bool_to_host(&lst.app(c_true()).app(cons_case))
}
fn head_l(lst: &D) -> D { lst.app(D::Str(String::new())).app(sel_k()) }
fn tail_l(lst: &D) -> D { lst.app(D::Str(String::new())).app(sel_ki()) }
fn walk_l(mut lst: D) -> Vec<D> {
    let mut out = Vec::new();
    while !is_nil(&lst) {
        out.push(head_l(&lst));
        lst = tail_l(&lst);
    }
    out
}

fn jInt(n: i64) -> D { mkpair(encodeInt(1), encodeInt(n)) }
fn jBool(b: D) -> D { mkpair(encodeInt(2), b) }
fn jStr(s: String) -> D {
    let mut lst = nil_h();
    for &byte in s.as_bytes().iter().rev() {
        lst = cons_h(encodeInt(byte as i64), lst);
    }
    mkpair(encodeInt(3), lst)
}
fn jArr(lst: D) -> D { mkpair(encodeInt(4), lst) }
fn jObj(lst: D) -> D { mkpair(encodeInt(5), lst) }
fn jNull() -> D { mkpair(encodeInt(6), lam(|x| x)) }

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

fn decodeJson(v: D) -> String {
    let tag = church_to_int(&fstp(&v));
    let payload = sndp(&v);
    match tag {
        1 => church_to_int(&payload).to_string(),
        2 => if bool_to_host(&payload) { "true".to_string() } else { "false".to_string() },
        3 => {
            let bytes: Vec<u8> = walk_l(payload).iter().map(|b| church_to_int(b) as u8).collect();
            json_escape(&String::from_utf8_lossy(&bytes))
        }
        4 => {
            let parts: Vec<String> = walk_l(payload).into_iter().map(decodeJson).collect();
            format!("[{}]", parts.join(","))
        }
        5 => {
            let parts: Vec<String> = walk_l(payload)
                .into_iter()
                .map(|pr| format!("{}:{}", decodeJson(fstp(&pr)), decodeJson(sndp(&pr))))
                .collect();
            format!("{{{}}}", parts.join(","))
        }
        6 => "null".to_string(),
        _ => "?".to_string(),
    }
}
