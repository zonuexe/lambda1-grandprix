//! Rust バックエンド（システム系・最難のクロージャ型付け）。
//!
//! 万能型は `enum D { Fun(Rc<dyn Fn(D)->D>), Num(i64), Str(String) }`。
//! 所有権対策として (1) 変数参照は毎回 `.clone()`、(2) 入れ子クロージャの前で
//! その自由変数を preclone する。このため `render` を term-aware に override する。

use super::Backend;
use crate::ast::Term;
use std::collections::BTreeSet;
use std::path::Path;
use std::process::{Command, ExitStatus};

pub struct Rust;

/// λ本体の自由変数（束縛変数を除く）。preclone 対象を決めるのに使う。
fn free_vars(t: &Term) -> BTreeSet<String> {
    fn go(t: &Term, bound: &mut Vec<String>, out: &mut BTreeSet<String>) {
        match t {
            Term::Var(v) => {
                if !bound.contains(v) {
                    out.insert(v.clone());
                }
            }
            Term::Lam(p, b) => {
                bound.push(p.clone());
                go(b, bound, out);
                bound.pop();
            }
            Term::App(f, x) => {
                go(f, bound, out);
                go(x, bound, out);
            }
            Term::HostCall(_, args) => {
                for a in args {
                    go(a, bound, out);
                }
            }
            Term::IntLit(_) | Term::StrLit(_) => {}
        }
    }
    let mut out = BTreeSet::new();
    let mut bound = Vec::new();
    go(t, &mut bound, &mut out);
    out
}

impl Backend for Rust {
    fn name(&self) -> &'static str {
        "rust"
    }
    fn ext(&self) -> &'static str {
        "rs"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/rust.rs")
    }

    // render を override するため emit_lam は未使用（trait 要件のため実装のみ）。
    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("lam(move |{}| {})", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("{}.app({})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        format!("{}({})", name, args.join(", "))
    }
    fn emit_str(&self, s: &str) -> String {
        format!("\"{}\".to_string()", s)
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("let {}: D = {};", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("_failures += check(\"assert {}\", {}, {});", idx + 1, lhs, rhs)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let defs_block = defs
            .iter()
            .map(|d| format!("    {}", d))
            .collect::<Vec<_>>()
            .join("\n");
        let asserts_block = asserts
            .iter()
            .map(|a| format!("    {}", a))
            .collect::<Vec<_>>()
            .join("\n");
        self.prelude()
            .replace("//__DEFS__", &defs_block)
            .replace("//__ASSERTS__", &asserts_block)
    }

    /// term-aware な描画。変数は clone、λ は自由変数を preclone してから move する。
    fn render(&self, t: &Term) -> String {
        match t {
            Term::Var(v) => format!("{}.clone()", self.mangle(v)),
            Term::Lam(p, body) => {
                let fvs = free_vars(t); // = fv(body) \ {p}
                let mut s = String::from("{ ");
                for v in &fvs {
                    let m = self.mangle(v);
                    s.push_str(&format!("let {m} = {m}.clone(); ", m = m));
                }
                s.push_str(&format!(
                    "lam(move |{}| {}) }}",
                    self.mangle(p),
                    self.render(body)
                ));
                s
            }
            Term::App(f, x) => self.emit_app(&self.render(f), &self.render(x)),
            Term::HostCall(n, args) => {
                let rendered: Vec<String> = args.iter().map(|a| self.render(a)).collect();
                self.emit_host_call(n, &rendered)
            }
            Term::IntLit(n) => self.emit_int(*n),
            Term::StrLit(s) => self.emit_str(s),
        }
    }

    fn exec(&self, dir: &Path, file: &Path) -> std::io::Result<ExitStatus> {
        let bin = dir.join("main_bin");
        let compile = Command::new("rustc")
            .arg("--edition")
            .arg("2021")
            .arg("-O")
            .arg(file)
            .arg("-o")
            .arg(&bin)
            .status()?;
        if !compile.success() {
            return Ok(compile);
        }
        Command::new(&bin).status()
    }
}
