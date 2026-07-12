//! newLISP / niiLISP バックエンド（cons 無し・ダイナミックスコープ）。
//!
//! ラムダ計算はレキシカルクロージャを要するが niiLISP はダイナミックスコープ。
//! kosh04 の手法を応用: `LAMBDA` マクロ（fexpr）が生成時に `expand` で「大文字の
//! 自由変数」を値に焼き込み、疑似レキシカルクロージャを作る。
//!
//! ただし `expand` は「今ダイナミックに束縛されている大文字シンボル」を焼き込むため、
//! 別の λ が同名の束縛変数を使っていると誤った値が焼き込まれる。これを避けるため
//! **束縛変数を全てグローバル一意名（V0, V1, …）にα変換**する（render を override）。
//! niiLISP は cons を持たないが church エンコードは cons 不使用。実行・検証は niiLISP
//! 実処理系（niilisp crate をラップした同梱 niilisp-runner）で行う（外部インタプリタは
//! LAM1_NEWLISP で差し替え可）。
//!
//! 既知の制約: `expand` は焼き込み時に大域 λ の本体（V 名を含む）を丸ごとインライン複製
//! するため、チャーチ前者関数 `pred` を深く入れ子で簡約する項（例 corpus/json.lam の
//! `range`）では複製された V 名が複数の動的フレームで衝突し評価に失敗する。RANGE 非依存の
//! 項（v1.lam 等）は全て緑。

use super::Backend;
use crate::ast::{Program, Term};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;

pub struct NewLisp {
    counter: RefCell<usize>,
}

impl NewLisp {
    pub fn new() -> Self {
        NewLisp {
            counter: RefCell::new(0),
        }
    }

    fn render_scoped(&self, t: &Term, scope: &HashMap<String, String>) -> String {
        match t {
            // scope にあれば束縛変数の一意名、無ければグローバル定義名（大文字）
            Term::Var(v) => scope.get(v).cloned().unwrap_or_else(|| self.mangle(v)),
            Term::Lam(p, body) => {
                let fresh = {
                    let mut c = self.counter.borrow_mut();
                    let n = *c;
                    *c += 1;
                    format!("V{}", n)
                };
                let mut s2 = scope.clone();
                s2.insert(p.clone(), fresh.clone());
                format!("(LAMBDA ({}) {})", fresh, self.render_scoped(body, &s2))
            }
            Term::App(f, x) => {
                format!(
                    "({} {})",
                    self.render_scoped(f, scope),
                    self.render_scoped(x, scope)
                )
            }
            Term::HostCall(n, args) => {
                let a: Vec<String> = args.iter().map(|x| self.render_scoped(x, scope)).collect();
                if a.is_empty() {
                    format!("({})", n)
                } else {
                    format!("({} {})", n, a.join(" "))
                }
            }
            Term::IntLit(n) => n.to_string(),
            Term::StrLit(s) => self.emit_str(s),
        }
    }
}

impl Backend for NewLisp {
    fn name(&self) -> &'static str {
        "newlisp"
    }
    fn ext(&self) -> &'static str {
        "lsp"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/newlisp.lsp")
    }
    fn library(&self) -> Option<(String, String)> {
        Some(("lam1.lsp".into(), self.prelude().into()))
    }

    // 大文字化（case-sensitive ゆえ小文字の newLISP 組込みと衝突しない）。
    fn mangle(&self, n: &str) -> String {
        n.to_uppercase()
    }

    fn emit_lam(&self, _param: &str, _body: &str) -> String {
        unreachable!("newlisp overrides render (alpha-renaming)")
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
        s.push_str("(load \"lam1.lsp\")  ; ヘルパー＋LAMBDA マクロは lam1.lsp\n");
        s.push_str("\n; --- definitions ---\n");
        for d in defs {
            s.push_str(d);
            s.push('\n');
        }
        s.push_str("\n; --- assertions ---\n");
        for a in asserts {
            s.push_str(a);
            s.push('\n');
        }
        s.push_str("\n(_finish)\n");
        s
    }

    fn render(&self, t: &Term) -> String {
        self.render_scoped(t, &HashMap::new())
    }

    fn generate(&self, prog: &Program) -> String {
        *self.counter.borrow_mut() = 0; // 決定的な V 番号のため毎回リセット
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

    fn run_argv(&self, _dir: &Path, file: &Path) -> Vec<String> {
        // 既定は niiLISP 実処理系（niilisp crate をラップした同梱 niilisp-runner）。
        // 外部インタプリタ（newLISP 等）と比較したい場合は LAM1_NEWLISP で差し替える。
        let runner = std::env::var("LAM1_NEWLISP").unwrap_or_else(|_| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("niilisp-runner")))
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|| "niilisp-runner".to_string())
        });
        vec![runner, file.to_string_lossy().into_owned()]
    }
}
