//! コード生成の共通基盤（ADR-0004 の T3 ハイブリッド）。
//!
//! 各言語は `Backend` trait を実装する。規則的な部分は `render`/`generate` の
//! デフォルト実装が担い、言語固有の描画だけを `emit_*` で与える。

use crate::ast::*;
use std::path::Path;
use std::process::{Command, Stdio};

pub mod clojure;
pub mod cpp;
pub mod emacs;
pub mod go;
pub mod haskell;
pub mod java;
pub mod kotlin;
pub mod lazyk;
pub mod pascal;
pub mod perl;
pub mod php;
pub mod python;
pub mod racket;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod sml;
pub mod swift;

pub trait Backend {
    fn name(&self) -> &'static str;
    fn ext(&self) -> &'static str;
    fn prelude(&self) -> &'static str;

    /// 変数・定義名のマングリング。予約語衝突を避けるため一律 `_` を前置する。
    fn mangle(&self, n: &str) -> String {
        format!("_{}", n)
    }

    fn emit_lam(&self, param: &str, body: &str) -> String;
    fn emit_app(&self, f: &str, x: &str) -> String;
    fn emit_host_call(&self, name: &str, args: &[String]) -> String;
    fn emit_int(&self, n: i64) -> String {
        n.to_string()
    }
    fn emit_str(&self, s: &str) -> String;
    fn emit_def(&self, name: &str, term: &str) -> String;
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String;
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String;

    /// コンパイル段（計測外）。インタプリタ言語は既定の no-op。成功なら true。
    fn build(&self, _dir: &Path, _file: &Path) -> std::io::Result<bool> {
        Ok(true)
    }

    /// 実行コマンド（計測対象）。`argv[0]` が実行ファイル、以降が引数。
    fn run_argv(&self, dir: &Path, file: &Path) -> Vec<String>;

    /// build して run する（`run` サブコマンド用）。成功＝全 assert 緑。
    fn exec(&self, dir: &Path, file: &Path) -> std::io::Result<bool> {
        if !self.build(dir, file)? {
            return Ok(false);
        }
        let argv = self.run_argv(dir, file);
        let st = Command::new(&argv[0]).args(&argv[1..]).status()?;
        Ok(st.success())
    }

    fn render(&self, t: &Term) -> String {
        match t {
            Term::Var(v) => self.mangle(v),
            Term::Lam(p, b) => {
                let body = self.render(b);
                self.emit_lam(&self.mangle(p), &body)
            }
            Term::App(f, x) => {
                let rf = self.render(f);
                let rx = self.render(x);
                self.emit_app(&rf, &rx)
            }
            Term::HostCall(n, args) => {
                let rendered: Vec<String> = args.iter().map(|a| self.render(a)).collect();
                self.emit_host_call(n, &rendered)
            }
            Term::IntLit(n) => self.emit_int(*n),
            Term::StrLit(s) => self.emit_str(s),
        }
    }

    fn generate(&self, prog: &Program) -> String {
        let defs: Vec<String> = prog
            .defs
            .iter()
            .map(|d| {
                let t = self.render(&d.term);
                self.emit_def(&self.mangle(&d.name), &t)
            })
            .collect();
        let asserts: Vec<String> = prog
            .asserts
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let l = self.render(&a.lhs);
                let r = self.render(&a.rhs);
                self.emit_assert(i, &l, &r)
            })
            .collect();
        self.emit_program(&defs, &asserts)
    }
}

/// v1 で有効なバックエンド一覧。
pub fn all_backends() -> Vec<Box<dyn Backend>> {
    vec![
        Box::new(python::Python),
        Box::new(racket::Racket),
        Box::new(haskell::Haskell),
        Box::new(java::Java),
        Box::new(rust::Rust),
        Box::new(ruby::Ruby),
        Box::new(php::Php),
        Box::new(clojure::Clojure),
        Box::new(emacs::Emacs),
        Box::new(go::Go),
        Box::new(sml::Sml),
        Box::new(perl::Perl),
        Box::new(scala::Scala),
        Box::new(kotlin::Kotlin),
        Box::new(cpp::Cpp::new()),
        Box::new(swift::Swift),
        Box::new(pascal::Pascal),
        Box::new(lazyk::LazyK),
    ]
}

/// 生成物をディレクトリに書き出し、(dir, file) を返す。
fn emit_to_dir(be: &dyn Backend, prog: &Program, outdir: &Path) -> std::io::Result<(std::path::PathBuf, std::path::PathBuf)> {
    let code = be.generate(prog);
    let dir = outdir.join(be.name());
    std::fs::create_dir_all(&dir)?;
    let file = dir.join(format!("main.{}", be.ext()));
    std::fs::write(&file, code)?;
    Ok((dir, file))
}

/// 1 バックエンド分を生成・実行する。戻り値は「全 assert 緑」なら true。
pub fn run_backend(be: &dyn Backend, prog: &Program, outdir: &Path) -> std::io::Result<bool> {
    let (dir, file) = emit_to_dir(be, prog, outdir)?;
    be.exec(&dir, &file)
}

/// 1 回の計測結果。
pub struct Sample {
    pub real_sec: f64,
    pub max_rss_bytes: u64,
}

/// 1 バックエンドをベンチする。build は計測外、run_argv の実行のみ `/usr/bin/time -l` で計測。
/// ノイズ低減のため 3 回実行し wall-clock 最小の回を採る（best-of-3）。
pub fn bench_backend(be: &dyn Backend, prog: &Program, outdir: &Path) -> std::io::Result<Option<Sample>> {
    let (dir, file) = emit_to_dir(be, prog, outdir)?;
    if !be.build(&dir, &file)? {
        return Ok(None);
    }
    let argv = be.run_argv(&dir, &file);
    let mut best: Option<Sample> = None;
    for _ in 0..3 {
        let out = Command::new("/usr/bin/time")
            .arg("-l")
            .args(&argv)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()?;
        if !out.status.success() {
            return Ok(None);
        }
        let stderr = String::from_utf8_lossy(&out.stderr);
        if let Some(s) = parse_time_l(&stderr) {
            match &best {
                Some(b) if b.real_sec <= s.real_sec => {}
                _ => best = Some(s),
            }
        }
    }
    Ok(best)
}

/// macOS の `/usr/bin/time -l` の stderr から (real 秒, 最大 RSS バイト) を取り出す。
fn parse_time_l(stderr: &str) -> Option<Sample> {
    let mut real = None;
    let mut rss = None;
    for line in stderr.lines() {
        let t = line.trim();
        if real.is_none() {
            if let Some(idx) = t.find(" real") {
                if let Some(tok) = t[..idx].split_whitespace().last() {
                    real = tok.parse::<f64>().ok();
                }
            }
        }
        if t.contains("maximum resident set size") {
            if let Some(tok) = t.split_whitespace().next() {
                rss = tok.parse::<u64>().ok();
            }
        }
    }
    Some(Sample {
        real_sec: real?,
        max_rss_bytes: rss?,
    })
}
