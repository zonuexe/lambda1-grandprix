//! Standard ML バックエンド（MLton）。万能型は datatype のタグ付き union。

use super::Backend;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub struct Sml;

impl Backend for Sml {
    fn name(&self) -> &'static str {
        "sml"
    }
    fn ext(&self) -> &'static str {
        "sml"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/sml.sml")
    }

    // SML の英数字識別子は文字始まり必須（`_x` は不正、`_` はワイルドカード）。
    // 予約語衝突は `v_` 前置で回避する。
    fn mangle(&self, n: &str) -> String {
        format!("v_{}", n)
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("(Fun (fn {} => {}))", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("(app {} {})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        if args.is_empty() {
            name.to_string()
        } else {
            format!("({} {})", name, args.join(" "))
        }
    }
    fn emit_str(&self, s: &str) -> String {
        format!("\"{}\"", s)
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("val {} = {}", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("val _ = check \"assert {}\" ({}) ({})", idx + 1, lhs, rhs)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let mut s = String::new();
        s.push_str(self.prelude());
        s.push_str("\n(* --- definitions --- *)\n");
        for d in defs {
            s.push_str(d);
            s.push('\n');
        }
        s.push_str("\n(* --- assertions --- *)\n");
        for a in asserts {
            s.push_str(a);
            s.push('\n');
        }
        s.push_str("\nval _ = finish ()\n");
        s
    }

    fn exec(&self, dir: &Path, file: &Path) -> std::io::Result<ExitStatus> {
        let bin = dir.join("main_bin");
        let compile = Command::new("mlton")
            .arg("-output")
            .arg(&bin)
            .arg(file)
            .status()?;
        if !compile.success() {
            return Ok(compile);
        }
        Command::new(&bin).status()
    }
}
