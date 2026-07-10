//! コード生成の共通基盤（ADR-0004 の T3 ハイブリッド）。
//!
//! 各言語は `Backend` trait を実装する。規則的な部分は `render`/`generate` の
//! デフォルト実装が担い、言語固有の描画だけを `emit_*` で与える。

use crate::ast::*;
use std::path::Path;
use std::process::ExitStatus;

pub mod haskell;
pub mod python;
pub mod racket;

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

    /// 生成物を実行し、終了ステータスを返す（成功＝全 assert 緑）。
    fn exec(&self, dir: &Path, file: &Path) -> std::io::Result<ExitStatus>;

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
    ]
}

/// 1 バックエンド分を生成・実行する。戻り値は「全 assert 緑」なら true。
pub fn run_backend(be: &dyn Backend, prog: &Program, outdir: &Path) -> std::io::Result<bool> {
    let code = be.generate(prog);
    let dir = outdir.join(be.name());
    std::fs::create_dir_all(&dir)?;
    let file = dir.join(format!("main.{}", be.ext()));
    std::fs::write(&file, code)?;
    let status = be.exec(&dir, &file)?;
    Ok(status.success())
}
