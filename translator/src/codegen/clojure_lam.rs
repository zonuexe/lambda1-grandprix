//! Clojure バックエンド（λ マクロ変種）。
//!
//! `#(...)` は入れ子不可でカリー化に使えないため、プレリュードで `λ` マクロ
//! （`(λ x body)` => `(fn [x] body)`）を自作し、λ抽象を `(λ x body)` と短く出力する。
//! それ以外は素の Clojure バックエンドと同じ。

use super::Backend;
use std::path::Path;

pub struct ClojureLam;

impl Backend for ClojureLam {
    fn name(&self) -> &'static str {
        "clojure-lam"
    }
    fn ext(&self) -> &'static str {
        "clj"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../../languages/clojure-lam/prelude.clj")
    }
    fn library(&self) -> Option<(String, String)> {
        Some(("lam1.clj".into(), self.prelude().into()))
    }

    fn reserved(&self) -> &'static [&'static str] {
        &[
            "def", "if", "do", "let", "quote", "var", "fn", "loop", "recur", "throw", "try",
            "catch", "finally", "new", "set!", "true", "false", "nil", "and", "or", "not",
        ]
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("(λ {} {})", param, body)
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
        format!("(def {} {})", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("(_check {} {} \"assert {}\")", lhs, rhs, idx + 1)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let mut s = String::new();
        s.push_str("(load-file \"lam1.clj\")  ; ヘルパー＋λマクロは lam1.clj\n");
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
        vec!["clojure".into(), "-M".into(), file.to_string_lossy().into_owned()]
    }
}
