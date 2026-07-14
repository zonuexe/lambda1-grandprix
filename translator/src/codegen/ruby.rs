//! Ruby バックエンド（動的）。万能型は不要。

use super::Backend;
use std::path::Path;

pub struct Ruby;

impl Backend for Ruby {
    fn name(&self) -> &'static str {
        "ruby"
    }
    fn ext(&self) -> &'static str {
        "rb"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../../languages/ruby/prelude.rb")
    }
    fn library(&self) -> Option<(String, String)> {
        Some(("lam1.rb".into(), self.prelude().into()))
    }

    fn reserved(&self) -> &'static [&'static str] {
        &[
            "BEGIN", "END", "alias", "and", "begin", "break", "case", "class", "def", "defined?",
            "do", "else", "elsif", "end", "ensure", "false", "for", "if", "in", "module", "next",
            "nil", "not", "or", "redo", "rescue", "retry", "return", "self", "super", "then",
            "true", "undef", "unless", "until", "when", "while", "yield",
        ]
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("(->({}) {{ {} }})", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("{}.({})", f, x)
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
        s.push_str("require_relative 'lam1'  # ヘルパーは lam1.rb\n");
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
        s.push_str("\n_finish\n");
        s
    }

    fn run_argv(&self, _dir: &Path, file: &Path) -> Vec<String> {
        let mut v = vec!["ruby".to_string()];
        if std::env::var("LAM1_JIT").is_ok() {
            v.push("--yjit".to_string()); // Ruby 3.x の YJIT
        }
        v.push(file.to_string_lossy().into_owned());
        v
    }
}
