//! λ-1 translator CLI.
//!
//!   translator run   [--lang <name>] <file.lam>   … 生成・実行して assert を確認
//!   translator bench [--lang <name>] <file.lam>   … 実行時間・最大 RSS を計測して比較
//!   translator gen   [--lang <name>] <file.lam>   … 読める成果物を demo/ に出力（実行しない）

mod ast;
mod codegen;
mod lexer;
mod parser;

use std::path::PathBuf;

enum Mode {
    Run,
    Bench,
    Gen,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut mode = Mode::Run;
    let mut lang: Option<String> = None;
    let mut out: Option<String> = None;
    let mut file: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "run" => mode = Mode::Run,
            "bench" => mode = Mode::Bench,
            "gen" => mode = Mode::Gen,
            "--lang" => {
                i += 1;
                lang = args.get(i).cloned();
            }
            "--out" => {
                i += 1;
                out = args.get(i).cloned();
            }
            other => file = Some(other.to_string()),
        }
        i += 1;
    }

    let file = match file {
        Some(f) => f,
        None => {
            eprintln!("usage: translator <run|bench> [--lang <name>] <file.lam>");
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
    let selected: Vec<&Box<dyn codegen::Backend>> = backends
        .iter()
        .filter(|be| lang.as_deref().map_or(true, |l| be.name() == l))
        .collect();

    if selected.is_empty() {
        eprintln!("no backend matched --lang {:?}", lang);
        std::process::exit(2);
    }

    match mode {
        Mode::Run => run(&selected, &prog, &outdir),
        Mode::Bench => bench(&selected, &prog, &outdir),
        Mode::Gen => gen(&selected, &prog, &src, out.as_deref().unwrap_or("demo")),
    }
}

/// 各言語の生成ソースを出力ディレクトリに書く（既定 demo/。聴衆が読む成果物・計測用）。
fn gen(selected: &[&Box<dyn codegen::Backend>], prog: &ast::Program, dsl_src: &str, out_root: &str) {
    let root = std::path::Path::new(out_root);
    for be in selected {
        let code = be.generate(prog);
        let dir = root.join(be.name());
        if let Err(e) = std::fs::create_dir_all(&dir) {
            eprintln!("{}: {}", be.name(), e);
            continue;
        }
        let file = dir.join(format!("demo.{}", be.ext()));
        match std::fs::write(&file, code) {
            Ok(()) => println!("wrote {}", file.display()),
            Err(e) => eprintln!("{}: {}", be.name(), e),
        }
        // ヘルパーを外部ライブラリに分離する言語は、そのファイルも並べて出力。
        if let Some((libname, libcode)) = be.library() {
            let libpath = dir.join(&libname);
            if std::fs::write(&libpath, libcode).is_ok() {
                println!("wrote {}", libpath.display());
            }
        }
    }
    // 元の DSL ソースも並べておく（対比用）。
    let src_path = root.join("source.lam");
    let _ = std::fs::write(&src_path, dsl_src);
    println!("wrote {}", src_path.display());
}

fn run(selected: &[&Box<dyn codegen::Backend>], prog: &ast::Program, outdir: &std::path::Path) {
    let mut any_fail = false;
    for be in selected {
        println!("=== {} ===", be.name());
        match codegen::run_backend(be.as_ref(), prog, outdir) {
            Ok(true) => {}
            Ok(false) => any_fail = true,
            Err(e) => {
                eprintln!("{} error: {}", be.name(), e);
                any_fail = true;
            }
        }
        println!();
    }
    std::process::exit(if any_fail { 1 } else { 0 });
}

fn bench(selected: &[&Box<dyn codegen::Backend>], prog: &ast::Program, outdir: &std::path::Path) {
    let mut rows: Vec<(String, Option<codegen::Sample>)> = Vec::new();
    for be in selected {
        eprintln!("benching {}…", be.name());
        let sample = codegen::bench_backend(be.as_ref(), prog, outdir).unwrap_or(None);
        rows.push((be.name().to_string(), sample));
    }

    // wall-clock 昇順（計測できなかったものは末尾）。
    rows.sort_by(|a, b| match (&a.1, &b.1) {
        (Some(x), Some(y)) => x.real_sec.partial_cmp(&y.real_sec).unwrap(),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });

    println!();
    println!("{:<10} {:>12} {:>12}", "language", "time (ms)", "peak (MB)");
    println!("{:-<10} {:->12} {:->12}", "", "", "");
    for (name, s) in &rows {
        match s {
            Some(s) => println!(
                "{:<10} {:>12.1} {:>12.1}",
                name,
                s.real_sec * 1000.0,
                s.max_rss_bytes as f64 / (1024.0 * 1024.0)
            ),
            None => println!("{:<10} {:>12} {:>12}", name, "n/a", "n/a"),
        }
    }
}
