//! Scala 3 バックエンド（JVM・型付き）。万能型は sealed trait の union。

use super::Backend;
use std::path::Path;
use std::process::Command;

pub struct Scala;

impl Backend for Scala {
    fn name(&self) -> &'static str {
        "scala"
    }
    fn ext(&self) -> &'static str {
        "scala"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/scala.scala")
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("Fun({} => {})", param, body)
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
        // @main run -> クラス run。scala runner が scala3 ライブラリを classpath に足す。
        vec![
            "scala".into(),
            "-classpath".into(),
            dir.to_string_lossy().into_owned(),
            "run".into(),
        ]
    }
}
