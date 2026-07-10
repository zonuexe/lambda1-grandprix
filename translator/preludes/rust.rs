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
//__DEFS__
//__ASSERTS__
    if _failures > 0 {
        println!("{} failure(s)", _failures);
        std::process::exit(1);
    }
    println!("all green");
}
