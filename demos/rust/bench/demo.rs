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
    let mult: D = { lam(move |m| { let m = m.clone(); lam(move |n| { let m = m.clone(); let n = n.clone(); lam(move |f| m.clone().app(n.clone().app(f.clone()))) }) }) };
    _failures += check("assert 1", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 2", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 3", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 4", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 5", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 6", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 7", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 8", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 9", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 10", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 11", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 12", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 13", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 14", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 15", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 16", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 17", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 18", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 19", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 20", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 21", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 22", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 23", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 24", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 25", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 26", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 27", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 28", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 29", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 30", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 31", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 32", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 33", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 34", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 35", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 36", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 37", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 38", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 39", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 40", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 41", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 42", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 43", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 44", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 45", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 46", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 47", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 48", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 49", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 50", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 51", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 52", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 53", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 54", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 55", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 56", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 57", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 58", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 59", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 60", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 61", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 62", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 63", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 64", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 65", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 66", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 67", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 68", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 69", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 70", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 71", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 72", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 73", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 74", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 75", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 76", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 77", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 78", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 79", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 80", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 81", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 82", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 83", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 84", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 85", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 86", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 87", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 88", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 89", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 90", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 91", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 92", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 93", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 94", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 95", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 96", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 97", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 98", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 99", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 100", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 101", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 102", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 103", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 104", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 105", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 106", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 107", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 108", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 109", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 110", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 111", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 112", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 113", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 114", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 115", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 116", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 117", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 118", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 119", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 120", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 121", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 122", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 123", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 124", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 125", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 126", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 127", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 128", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 129", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 130", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 131", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 132", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 133", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 134", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 135", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 136", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 137", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 138", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 139", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 140", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 141", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 142", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 143", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 144", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 145", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 146", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 147", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 148", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 149", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 150", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 151", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 152", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 153", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 154", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 155", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 156", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 157", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 158", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 159", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 160", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 161", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 162", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 163", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 164", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 165", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 166", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 167", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 168", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 169", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 170", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 171", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 172", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 173", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 174", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 175", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 176", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 177", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 178", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 179", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 180", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 181", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 182", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 183", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 184", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 185", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 186", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 187", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 188", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 189", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 190", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 191", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 192", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 193", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 194", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 195", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 196", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 197", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 198", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 199", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 200", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 201", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 202", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 203", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 204", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 205", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 206", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 207", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 208", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 209", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 210", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 211", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 212", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 213", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 214", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 215", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 216", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 217", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 218", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 219", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 220", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 221", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 222", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 223", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 224", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 225", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 226", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 227", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 228", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 229", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 230", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 231", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 232", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 233", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 234", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 235", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 236", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 237", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 238", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 239", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 240", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 241", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 242", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 243", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 244", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 245", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 246", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 247", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 248", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 249", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 250", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 251", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 252", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 253", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 254", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 255", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 256", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 257", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 258", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 259", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 260", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 261", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 262", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 263", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 264", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 265", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 266", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 267", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 268", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 269", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 270", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 271", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 272", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 273", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 274", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 275", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 276", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 277", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 278", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 279", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 280", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 281", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 282", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 283", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 284", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 285", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 286", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 287", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 288", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 289", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 290", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 291", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 292", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 293", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 294", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 295", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 296", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 297", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 298", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 299", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 300", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 301", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 302", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 303", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 304", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 305", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 306", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 307", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 308", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 309", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 310", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 311", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 312", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 313", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 314", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 315", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 316", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 317", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 318", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 319", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 320", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 321", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 322", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 323", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 324", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 325", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 326", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 327", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 328", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 329", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 330", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 331", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 332", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 333", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 334", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 335", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 336", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 337", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 338", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 339", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 340", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 341", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 342", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 343", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 344", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 345", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 346", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 347", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 348", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 349", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 350", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 351", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 352", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 353", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 354", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 355", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 356", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 357", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 358", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 359", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 360", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 361", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 362", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 363", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 364", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 365", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 366", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 367", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 368", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 369", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 370", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 371", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 372", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 373", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 374", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 375", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 376", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 377", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 378", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 379", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 380", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 381", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 382", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 383", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 384", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 385", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 386", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 387", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 388", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 389", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 390", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 391", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 392", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 393", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 394", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 395", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 396", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 397", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 398", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 399", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 400", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 401", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 402", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 403", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 404", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 405", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 406", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 407", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 408", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 409", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 410", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 411", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 412", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 413", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 414", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 415", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 416", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 417", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 418", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 419", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 420", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 421", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 422", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 423", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 424", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 425", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 426", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 427", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 428", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 429", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 430", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 431", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 432", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 433", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 434", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 435", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 436", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 437", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 438", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 439", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 440", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 441", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 442", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 443", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 444", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 445", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 446", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 447", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 448", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 449", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 450", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 451", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 452", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 453", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 454", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 455", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 456", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 457", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 458", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 459", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 460", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 461", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 462", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 463", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 464", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 465", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 466", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 467", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 468", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 469", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 470", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 471", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 472", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 473", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 474", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 475", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 476", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 477", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 478", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 479", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 480", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 481", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 482", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 483", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 484", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 485", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 486", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 487", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 488", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 489", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 490", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 491", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 492", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 493", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 494", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 495", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 496", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 497", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 498", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 499", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
    _failures += check("assert 500", "900".to_string(), decodeInt(mult.clone().app(encodeInt(30)).app(encodeInt(30))));
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
