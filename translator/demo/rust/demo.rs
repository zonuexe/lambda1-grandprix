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
    let _I: D = { lam(move |_x| _x.clone()) };
    let _K: D = { lam(move |_x| { let _x = _x.clone(); lam(move |_y| _x.clone()) }) };
    let _S: D = { lam(move |_x| { let _x = _x.clone(); lam(move |_y| { let _x = _x.clone(); let _y = _y.clone(); lam(move |_z| _x.clone().app(_z.clone()).app(_y.clone().app(_z.clone()))) }) }) };
    let _zero: D = { lam(move |_f| { lam(move |_x| _x.clone()) }) };
    let _succ: D = { lam(move |_n| { let _n = _n.clone(); lam(move |_f| { let _f = _f.clone(); let _n = _n.clone(); lam(move |_x| _f.clone().app(_n.clone().app(_f.clone()).app(_x.clone()))) }) }) };
    let _add: D = { lam(move |_m| { let _m = _m.clone(); lam(move |_n| { let _m = _m.clone(); let _n = _n.clone(); lam(move |_f| { let _f = _f.clone(); let _m = _m.clone(); let _n = _n.clone(); lam(move |_x| _m.clone().app(_f.clone()).app(_n.clone().app(_f.clone()).app(_x.clone()))) }) }) }) };
    let _true: D = { lam(move |_t| { let _t = _t.clone(); lam(move |_f| _t.clone()) }) };
    let _false: D = { lam(move |_t| { lam(move |_f| _f.clone()) }) };
    let _if: D = { lam(move |_b| { let _b = _b.clone(); lam(move |_t| { let _b = _b.clone(); let _t = _t.clone(); lam(move |_e| _b.clone().app(_t.clone()).app(_e.clone())) }) }) };
    let _and: D = { lam(move |_p| { let _p = _p.clone(); lam(move |_q| _p.clone().app(_q.clone()).app(_p.clone())) }) };
    let _not: D = { let _false = _false.clone(); let _true = _true.clone(); lam(move |_b| _b.clone().app(_false.clone()).app(_true.clone())) };
    _failures += check("assert 1", "1".to_string(), decodeInt(_S.clone().app(_K.clone()).app(_K.clone()).app(encodeInt(1))));
    _failures += check("assert 2", "0".to_string(), decodeInt(_zero.clone()));
    _failures += check("assert 3", "3".to_string(), decodeInt(_succ.clone().app(encodeInt(2))));
    _failures += check("assert 4", "2".to_string(), decodeInt(_add.clone().app(encodeInt(1)).app(encodeInt(1))));
    _failures += check("assert 5", "true".to_string(), decodeBool(_and.clone().app(_true.clone()).app(_true.clone())));
    _failures += check("assert 6", "false".to_string(), decodeBool(_and.clone().app(_true.clone()).app(_false.clone())));
    _failures += check("assert 7", "false".to_string(), decodeBool(_if.clone().app(_false.clone()).app(_true.clone()).app(_false.clone())));
    _failures += check("assert 8", "true".to_string(), decodeBool(_not.clone().app(_false.clone())));
    if _failures > 0 {
        println!("{} failure(s)", _failures);
        std::process::exit(1);
    }
    println!("all green");
}
