//! Racket バックエンド（Lisp-1・sexpr・動的）。万能型は不要（素のクロージャが値）。

use super::Backend;
use std::path::Path;

pub struct Racket;

impl Backend for Racket {
    fn name(&self) -> &'static str {
        "racket"
    }
    fn ext(&self) -> &'static str {
        "rkt"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/racket.rkt")
    }
    fn library(&self) -> Option<(String, String)> {
        Some(("lam1.rkt".into(), self.prelude().into()))
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("(lambda ({}) {})", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("({} {})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        if args.is_empty() {
            format!("({})", name)
        } else {
            format!("({} {})", name, args.join(" "))
        }
    }
    fn emit_str(&self, s: &str) -> String {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("(define {} {})", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("(_check {} {} \"assert {}\")", lhs, rhs, idx + 1)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let mut s = String::new();
        s.push_str("#lang racket\n(require \"lam1.rkt\")  ; ヘルパーは lam1.rkt\n");
        s.push_str("\n;; --- definitions ---\n");
        for d in defs {
            s.push_str(d);
            s.push('\n');
        }
        s.push_str("\n;; --- assertions ---\n");
        for a in asserts {
            s.push_str(a);
            s.push('\n');
        }
        s.push_str("\n(_finish)\n");
        s
    }

    fn run_argv(&self, _dir: &Path, file: &Path) -> Vec<String> {
        vec!["racket".into(), file.to_string_lossy().into_owned()]
    }
}
