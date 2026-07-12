//! Python バックエンド（動的・基準実装）。万能型は不要で素のクロージャが値。

use super::Backend;
use std::path::Path;

pub struct Python;

impl Backend for Python {
    fn name(&self) -> &'static str {
        "python"
    }
    fn ext(&self) -> &'static str {
        "py"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/python.py")
    }
    fn library(&self) -> Option<(String, String)> {
        Some(("lam1.py".into(), self.prelude().into()))
    }

    fn reserved(&self) -> &'static [&'static str] {
        &[
            "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class",
            "continue", "def", "del", "elif", "else", "except", "finally", "for", "from", "global",
            "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise",
            "return", "try", "while", "with", "yield",
        ]
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        // 適用時に body が誤結合しないよう常に括弧で包む
        format!("(lambda {}: {})", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("{}({})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        format!("{}({})", name, args.join(", "))
    }
    fn emit_str(&self, s: &str) -> String {
        format!("'{}'", s)
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("{} = {}", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("_check({}, {}, 'assert {}')", lhs, rhs, idx + 1)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let mut s = String::new();
        s.push_str("from lam1 import *  # ヘルパーは lam1.py\n");
        s.push_str("\n# --- definitions ---\n");
        for d in defs {
            s.push_str(d);
            s.push('\n');
        }
        s.push_str("\n# --- assertions ---\n");
        for a in asserts {
            s.push_str(a);
            s.push('\n');
        }
        s.push_str("\n_finish()\n");
        s
    }

    fn run_argv(&self, _dir: &Path, file: &Path) -> Vec<String> {
        vec!["python3".into(), file.to_string_lossy().into_owned()]
    }
}
