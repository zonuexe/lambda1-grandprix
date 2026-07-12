//! Scala 3 バックエンド（中置 `~` 適用演算子の変種）。
//!
//! 万能型 D に中置演算子 `def ~ (x: D): D` を定義し、λ適用を `f ~ (x)` と中置で書く
//! （`app(f, x)` の代わり）。カリー化した適用が `f ~ x ~ y` と演算子チェーンで読める。
//! それ以外は素の Scala バックエンドと同じ。

use super::Backend;
use std::path::Path;
use std::process::Command;

pub struct ScalaInfix;

impl Backend for ScalaInfix {
    fn name(&self) -> &'static str {
        "scala-infix"
    }
    fn ext(&self) -> &'static str {
        "scala"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/scala_infix.scala")
    }

    fn reserved(&self) -> &'static [&'static str] {
        &[
            "abstract", "case", "catch", "class", "def", "do", "else", "enum", "export", "extends",
            "false", "final", "finally", "for", "forSome", "given", "if", "implicit", "import",
            "lazy", "match", "new", "null", "object", "override", "package", "private", "protected",
            "return", "sealed", "super", "then", "throw", "trait", "true", "try", "type", "val",
            "var", "while", "with", "yield",
        ]
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("Fun({} => {})", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        // 中置適用。右オペランドは常に括弧で包み左結合チェーンを保つ。
        format!("{} ~ ({})", f, x)
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
            .map(|a| format!("  {}", a))
            .collect::<Vec<_>>()
            .join("\n");
        self.prelude()
            .replace("//__DEFS__", &defs_block)
            .replace("//__ASSERTS__", &asserts_block)
    }

    fn build(&self, dir: &Path, file: &Path) -> std::io::Result<bool> {
        let st = Command::new("scalac").arg("-d").arg(dir).arg(file).status()?;
        Ok(st.success())
    }
    fn run_argv(&self, dir: &Path, _file: &Path) -> Vec<String> {
        vec![
            "scala".into(),
            "-classpath".into(),
            dir.to_string_lossy().into_owned(),
            "run".into(),
        ]
    }
}
