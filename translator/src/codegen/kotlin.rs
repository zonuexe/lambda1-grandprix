//! Kotlin バックエンド（JVM・型付き）。万能型は sealed class の union。

use super::Backend;
use std::path::Path;
use std::process::Command;

pub struct Kotlin;

impl Backend for Kotlin {
    fn name(&self) -> &'static str {
        "kotlin"
    }
    fn ext(&self) -> &'static str {
        "kt"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/kotlin.kt")
    }

    fn reserved(&self) -> &'static [&'static str] {
        &[
            "as", "break", "class", "continue", "do", "else", "false", "for", "fun", "if", "in",
            "interface", "is", "null", "object", "package", "return", "super", "this", "throw",
            "true", "try", "typealias", "typeof", "val", "var", "when", "while",
        ]
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("Fun({{ {} -> {} }})", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("app({}, {})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        format!("{}({})", name, args.join(", "))
    }
    fn emit_str(&self, s: &str) -> String {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("val {}: D = {}", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("check(\"assert {}\", {}, {})", idx + 1, lhs, rhs)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let defs_block = defs.join("\n");
        let asserts_block = asserts
            .iter()
            .map(|a| format!("    {}", a))
            .collect::<Vec<_>>()
            .join("\n");
        self.prelude()
            .replace("//__DEFS__", &defs_block)
            .replace("//__ASSERTS__", &asserts_block)
    }

    fn build(&self, dir: &Path, file: &Path) -> std::io::Result<bool> {
        let jar = dir.join("app.jar");
        let st = Command::new("kotlinc")
            .arg(file)
            .arg("-include-runtime")
            .arg("-d")
            .arg(&jar)
            .status()?;
        Ok(st.success())
    }
    fn run_argv(&self, dir: &Path, _file: &Path) -> Vec<String> {
        vec![
            "java".into(),
            "-jar".into(),
            dir.join("app.jar").to_string_lossy().into_owned(),
        ]
    }
}
