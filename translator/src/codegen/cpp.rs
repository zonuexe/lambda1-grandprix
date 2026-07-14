//! C++ バックエンド（システム clang++・型付き）。
//! 万能型は `std::variant<Fn, long, std::string>`。C++ ラムダは明示キャプチャが要るので、
//! 各 λ の自由変数を capture list に出すため `render` を term-aware に override する。
//! ただしトップレベル定義（グローバル）は静的記憶域でキャプチャ不可かつ不要なので除外する。

use super::Backend;
use crate::ast::{Program, Term};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

pub struct Cpp {
    globals: RefCell<BTreeSet<String>>,
}

impl Cpp {
    pub fn new() -> Self {
        Cpp {
            globals: RefCell::new(BTreeSet::new()),
        }
    }
}

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

impl Backend for Cpp {
    fn name(&self) -> &'static str {
        "cpp"
    }
    fn ext(&self) -> &'static str {
        "cpp"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../../languages/cpp/prelude.cpp")
    }

    fn reserved(&self) -> &'static [&'static str] {
        &[
            "alignas", "alignof", "and", "and_eq", "asm", "auto", "bitand", "bitor", "bool",
            "break", "case", "catch", "char", "class", "compl", "const", "constexpr", "continue",
            "decltype", "default", "delete", "do", "double", "dynamic_cast", "else", "enum",
            "explicit", "export", "extern", "false", "float", "for", "friend", "goto", "if",
            "inline", "int", "long", "mutable", "namespace", "new", "noexcept", "not", "not_eq",
            "nullptr", "operator", "or", "or_eq", "private", "protected", "public", "register",
            "return", "short", "signed", "sizeof", "static", "static_cast", "struct", "switch",
            "template", "this", "throw", "true", "try", "typedef", "typeid", "typename", "union",
            "unsigned", "using", "virtual", "void", "volatile", "while", "xor", "xor_eq",
        ]
    }

    fn emit_lam(&self, _param: &str, _body: &str) -> String {
        unreachable!("cpp overrides render")
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("{}.apply({})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        format!("{}({})", name, args.join(", "))
    }
    fn emit_str(&self, s: &str) -> String {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("D {} = {};", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("check(\"assert {}\", {}, {});", idx + 1, lhs, rhs)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let defs_block = defs.join("\n");
        let asserts_block = asserts
            .iter()
            .map(|a| format!("    {}", a))
            .collect::<Vec<_>>()
            .join("\n");
        self.prelude()
            .replace("//__DEFS__", &defs_block)
            .replace("//__ASSERTS__", &asserts_block)
    }

    // 定義名（グローバル）を先に集めてから通常の生成を行う。
    fn generate(&self, prog: &Program) -> String {
        *self.globals.borrow_mut() = prog.defs.iter().map(|d| self.mangle(&d.name)).collect();
        let defs: Vec<String> = prog
            .defs
            .iter()
            .map(|d| {
                let t = self.render(&d.term);
                self.emit_def(&self.mangle(&d.name), &t)
            })
            .collect();
        let asserts: Vec<String> = prog
            .asserts
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let l = self.render(&a.lhs);
                let r = self.render(&a.rhs);
                self.emit_assert(i, &l, &r)
            })
            .collect();
        self.emit_program(&defs, &asserts)
    }

    fn render(&self, t: &Term) -> String {
        match t {
            Term::Var(v) => self.mangle(v),
            Term::Lam(p, body) => {
                let globals = self.globals.borrow();
                let cap = free_vars(t)
                    .into_iter()
                    .map(|v| self.mangle(&v))
                    .filter(|m| !globals.contains(m)) // グローバルはキャプチャ不可・不要
                    .collect::<Vec<_>>()
                    .join(", ");
                drop(globals);
                format!(
                    "D(Fn([{}](D {}) {{ return {}; }}))",
                    cap,
                    self.mangle(p),
                    self.render(body)
                )
            }
            Term::App(f, x) => self.emit_app(&self.render(f), &self.render(x)),
            Term::HostCall(n, args) => {
                let a: Vec<String> = args.iter().map(|x| self.render(x)).collect();
                self.emit_host_call(n, &a)
            }
            Term::IntLit(n) => self.emit_int(*n),
            Term::StrLit(s) => self.emit_str(s),
        }
    }

    fn build(&self, dir: &Path, file: &Path) -> std::io::Result<bool> {
        let bin = dir.join("main_bin");
        let st = Command::new("/usr/bin/clang++")
            .arg("-std=c++17")
            .arg("-O2")
            .arg(file)
            .arg("-o")
            .arg(&bin)
            .status()?;
        Ok(st.success())
    }
    fn run_argv(&self, dir: &Path, _file: &Path) -> Vec<String> {
        vec![dir.join("main_bin").to_string_lossy().into_owned()]
    }
}
