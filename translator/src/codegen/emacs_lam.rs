//! Emacs Lisp バックエンド（λ マクロ変種）。
//!
//! プレリュードで `λ` マクロ（`(λ x body)` => `(lambda (x) body)`）を自作し、
//! λ抽象を `(λ x body)` と短く出力する。それ以外は素の Emacs バックエンドと同じ
//! （Lisp-2 なので適用は `(funcall f x)`）。
//! ※ `(λ x . body)` の `.` は Lisp のドット対と衝突するため採用できない（実機で確認済み）。

use super::Backend;
use std::path::Path;

pub struct EmacsLam;

impl Backend for EmacsLam {
    fn name(&self) -> &'static str {
        "emacs-lam"
    }
    fn ext(&self) -> &'static str {
        "el"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/emacs_lam.el")
    }
    fn library(&self) -> Option<(String, String)> {
        Some(("lam1.el".into(), self.prelude().into()))
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("(λ {} {})", param, body)
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
        s.push_str(";;; -*- lexical-binding: t; -*-\n(load-file \"lam1.el\")  ; ヘルパー＋λマクロは lam1.el\n");
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
        vec![
            "emacs".into(),
            "--script".into(),
            file.to_string_lossy().into_owned(),
        ]
    }
}
