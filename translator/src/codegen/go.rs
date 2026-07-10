//! Go バックエンド（型付き）。万能型は interface のタグ付き union（Fun/Num/Str）。

use super::Backend;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub struct Go;

impl Backend for Go {
    fn name(&self) -> &'static str {
        "go"
    }
    fn ext(&self) -> &'static str {
        "go"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/go.go")
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("Fun(func({} D) D {{ return {} }})", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("app({}, {})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        format!("{}({})", name, args.join(", "))
    }
    fn emit_str(&self, s: &str) -> String {
        format!("\"{}\"", s)
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("var {} D = {}", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("check(\"assert {}\", {}, {})", idx + 1, lhs, rhs)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let defs_block = defs.join("\n");
        let asserts_block = asserts
            .iter()
            .map(|a| format!("\t{}", a))
            .collect::<Vec<_>>()
            .join("\n");
        self.prelude()
            .replace("//__DEFS__", &defs_block)
            .replace("//__ASSERTS__", &asserts_block)
    }

    fn exec(&self, _dir: &Path, file: &Path) -> std::io::Result<ExitStatus> {
        Command::new("go").arg("run").arg(file).status()
    }
}
