//! λ-1 translator CLI.
//!
//!   translator run [--lang <name>] <file.lam>

mod ast;
mod codegen;
mod lexer;
mod parser;

use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut lang: Option<String> = None;
    let mut file: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "run" => {}
            "--lang" => {
                i += 1;
                lang = args.get(i).cloned();
            }
            other => file = Some(other.to_string()),
        }
        i += 1;
    }

    let file = match file {
        Some(f) => f,
        None => {
            eprintln!("usage: translator run [--lang <name>] <file.lam>");
            std::process::exit(2);
        }
    };

    let src = std::fs::read_to_string(&file).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {}", file, e);
        std::process::exit(2);
    });

    let toks = lexer::lex(&src).unwrap_or_else(|e| {
        eprintln!("lex error: {}", e);
        std::process::exit(1);
    });
    let prog = parser::Parser::new(toks)
        .parse_program()
        .unwrap_or_else(|e| {
            eprintln!("parse error: {}", e);
            std::process::exit(1);
        });

    let outdir = PathBuf::from(".gen");
    let backends = codegen::all_backends();
    let mut any_fail = false;
    let mut ran = 0;

    for be in &backends {
        if let Some(l) = &lang {
            if be.name() != l {
                continue;
            }
        }
        ran += 1;
        println!("=== {} ===", be.name());
        match codegen::run_backend(be.as_ref(), &prog, &outdir) {
            Ok(true) => {}
            Ok(false) => any_fail = true,
            Err(e) => {
                eprintln!("{} error: {}", be.name(), e);
                any_fail = true;
            }
        }
        println!();
    }

    if ran == 0 {
        eprintln!("no backend matched --lang {:?}", lang);
        std::process::exit(2);
    }
    std::process::exit(if any_fail { 1 } else { 0 });
}
