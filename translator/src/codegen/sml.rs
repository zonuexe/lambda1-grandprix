//! Standard ML バックエンド（MLton）。万能型は datatype のタグ付き union。

use super::Backend;
use std::path::Path;
use std::process::Command;

pub struct Sml;

impl Backend for Sml {
    fn name(&self) -> &'static str {
        "sml"
    }
    fn ext(&self) -> &'static str {
        "sml"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/sml.sml")
    }

    // SML キーワード＋組込み構築子 true/false/nil。DSL 名がこれに一致した時だけエスケープ。
    // 大文字始まりの識別子（I/K/S/Z 等）は SML では通常の値識別子として合法なのでそのまま。
    fn reserved(&self) -> &'static [&'static str] {
        &[
            "abstype", "and", "andalso", "as", "case", "datatype", "do", "else", "end", "eqtype",
            "exception", "fn", "fun", "functor", "handle", "if", "in", "include", "infix",
            "infixr", "let", "local", "nonfix", "of", "op", "open", "orelse", "raise", "rec",
            "sharing", "sig", "signature", "struct", "structure", "then", "type", "val", "where",
            "while", "with", "withtype", "true", "false", "nil",
        ]
    }

    // SML の英数字識別子は文字始まり必須（`_x` は不正、`_` はワイルドカード）ので、
    // 予約語衝突は `_` ではなく `v_` 前置で回避する。
    fn mangle(&self, n: &str) -> String {
        if self.reserved().contains(&n) {
            format!("v_{}", n)
        } else {
            n.to_string()
        }
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("(Fun (fn {} => {}))", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("(app {} {})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        if args.is_empty() {
            name.to_string()
        } else {
            format!("({} {})", name, args.join(" "))
        }
    }
    fn emit_str(&self, s: &str) -> String {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("val {} = {}", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("val _ = check \"assert {}\" ({}) ({})", idx + 1, lhs, rhs)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let mut s = String::new();
        s.push_str(self.prelude());
        s.push_str("\n(* --- definitions --- *)\n");
        for d in defs {
            s.push_str(d);
            s.push('\n');
        }
        s.push_str("\n(* --- assertions --- *)\n");
        for a in asserts {
            s.push_str(a);
            s.push('\n');
        }
        s.push_str("\nval _ = finish ()\n");
        s
    }

    fn build(&self, dir: &Path, file: &Path) -> std::io::Result<bool> {
        let bin = dir.join("main_bin");
        let st = Command::new("mlton")
            .arg("-output")
            .arg(&bin)
            .arg(file)
            .status()?;
        Ok(st.success())
    }
    fn run_argv(&self, dir: &Path, _file: &Path) -> Vec<String> {
        vec![dir.join("main_bin").to_string_lossy().into_owned()]
    }
}
