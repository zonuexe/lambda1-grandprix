//! Lazy K ランナー。Lazy K プログラム（SKI 等の項）を zonu-lazyk crate で実行する。
//! stdin をプログラムへ、プログラムの出力を stdout へ流す。

use std::io::{stdin, stdout};

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: lazyk-runner <file.lazy>");
            std::process::exit(2);
        }
    };
    let src = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("lazyk-runner: cannot read {path}: {e}");
        std::process::exit(2);
    });

    match zonu_lazyk::run(&src, stdin().lock(), stdout().lock()) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("lazyk-runner: {e:?}");
            std::process::exit(1);
        }
    }
}
