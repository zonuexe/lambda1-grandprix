//! Swift バックエンド（システム swift・型付き）。万能型は enum の union。
//! Swift クロージャは自動キャプチャなので capture list 不要（default render で足る）。

use super::Backend;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub struct Swift;

impl Backend for Swift {
    fn name(&self) -> &'static str {
        "swift"
    }
    fn ext(&self) -> &'static str {
        "swift"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/swift.swift")
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!(".fun {{ {} in {} }}", param, body)
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
        format!("let {}: D = {}", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("check(\"assert {}\", {}, {})", idx + 1, lhs, rhs)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let defs_block = defs.join("\n");
        let asserts_block = asserts.join("\n");
        self.prelude()
            .replace("//__DEFS__", &defs_block)
            .replace("//__ASSERTS__", &asserts_block)
    }

    fn exec(&self, _dir: &Path, file: &Path) -> std::io::Result<ExitStatus> {
        // nix devShell の SDKROOT/DEVELOPER_DIR は nix 提供の古い SDK を指し、
        // システム swift(6.3.x) と不一致になる。除去して既定 SDK を使わせる。
        Command::new("/usr/bin/swift")
            .arg(file)
            .env_remove("SDKROOT")
            .env_remove("DEVELOPER_DIR")
            .status()
    }
}
