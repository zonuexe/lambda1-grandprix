//! Emacs Lisp バックエンド（Lisp-2・動的）。
//!
//! Lisp-2 なので λ適用は `(funcall f x)`、ホスト呼び出しは `(name args)`（関数位置）。
//! DSL の「並置＝適用／`name[…]`＝ホスト」の区別がそのまま funcall の有無に対応する。

use super::Backend;
use std::path::Path;

pub struct Emacs;

impl Backend for Emacs {
    fn name(&self) -> &'static str {
        "emacs"
    }
    fn ext(&self) -> &'static str {
        "el"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/emacs.el")
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("(lambda ({}) {})", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("(funcall {} {})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        if args.is_empty() {
            format!("({})", name)
        } else {
            format!("({} {})", name, args.join(" "))
        }
    }
    fn emit_str(&self, s: &str) -> String {
        format!("\"{}\"", s)
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("(setq {} {})", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("(_check {} {} \"assert {}\")", lhs, rhs, idx + 1)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let mut s = String::new();
        s.push_str(self.prelude()); // 先頭に lexical-binding ヘッダを含む
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
        vec!["emacs".into(), "--script".into(), file.to_string_lossy().into_owned()]
    }
}
