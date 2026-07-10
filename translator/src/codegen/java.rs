//! Java バックエンド（型付き OO・関数型インタフェース）。
//! 万能型はクラス階層 D / Fun / Num / Str（タグ付き union）で表現する。

use super::Backend;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub struct Java;

impl Backend for Java {
    fn name(&self) -> &'static str {
        "java"
    }
    fn ext(&self) -> &'static str {
        "java"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/java.java")
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("new Fun({} -> {})", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("{}.apply({})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        format!("{}({})", name, args.join(", "))
    }
    fn emit_str(&self, s: &str) -> String {
        format!("\"{}\"", s)
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("static final D {} = {};", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("check(\"assert {}\", {}, {});", idx + 1, lhs, rhs)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let defs_block = defs
            .iter()
            .map(|d| format!("    {}", d))
            .collect::<Vec<_>>()
            .join("\n");
        let asserts_block = asserts
            .iter()
            .map(|a| format!("        {}", a))
            .collect::<Vec<_>>()
            .join("\n");
        self.prelude()
            .replace("//__DEFS__", &defs_block)
            .replace("//__ASSERTS__", &asserts_block)
    }

    fn exec(&self, _dir: &Path, file: &Path) -> std::io::Result<ExitStatus> {
        // Java 11+ の単一ファイルソース実行（javac 不要）
        Command::new("java").arg(file).status()
    }
}
