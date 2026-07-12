//! newLISP / niiLISP バックエンド（cons 有り・ダイナミックスコープ）。
//!
//! niiLISP/newLISP はダイナミックスコープでレキシカルクロージャを持たない。ネストした
//! ネイティブ lambda に自由変数を持たせると、適用時に束縛が壊れる。そこで **クロージャ変換
//! （defunctionalization / issues/14 approach C）** を採る:
//!
//!   - 各 λ を「トップレベル関数 `Lk(env x)`（自由変数ゼロ）＋捕捉環境データ」に変換する。
//!   - クロージャ値は `(list Lk cap...)`（関数シンボル＋捕捉値の列）というデータ。
//!   - 適用は一律 `(APPLY clo x)`。ネイティブ関数（プレリュードが注入する `inc` 等）も
//!     `APPLY` が扱う。
//!
//! ネストしたネイティブ lambda を一切作らないため、動的スコープに依存せず決定的に正しい。
//! 大域定義は安定なので捕捉せず `g_名` で名前参照する。プレリュード（`APPLY`/`encodeInt`/
//! JSON 層など）も同じ defunctionalized 表現で書く。

use super::Backend;
use crate::ast::{Program, Term};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;

pub struct NewLisp;

impl NewLisp {
    pub fn new() -> Self {
        NewLisp
    }
}

fn nl_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// λ の自由変数（束縛変数と大域を除く）。出現順・重複なし。
fn free_vars(t: &Term, globals: &BTreeSet<String>) -> Vec<String> {
    fn go(t: &Term, bound: &mut Vec<String>, out: &mut Vec<String>, globals: &BTreeSet<String>) {
        match t {
            Term::Var(v) => {
                if !bound.contains(v) && !globals.contains(v) && !out.contains(v) {
                    out.push(v.clone());
                }
            }
            Term::Lam(p, b) => {
                bound.push(p.clone());
                go(b, bound, out, globals);
                bound.pop();
            }
            Term::App(f, x) => {
                go(f, bound, out, globals);
                go(x, bound, out, globals);
            }
            Term::HostCall(_, args) => {
                for a in args {
                    go(a, bound, out, globals);
                }
            }
            Term::IntLit(_) | Term::StrLit(_) => {}
        }
    }
    let mut out = Vec::new();
    let mut bound = Vec::new();
    go(t, &mut bound, &mut out, globals);
    out
}

/// クロージャ変換の作業状態。生成した `Lk` 関数定義を貯める。
struct Conv<'a> {
    defs: Vec<String>,
    lc: usize, // Lk 関数名カウンタ
    vc: usize, // 変数名（param / captured）カウンタ
    globals: &'a BTreeSet<String>,
}

impl<'a> Conv<'a> {
    fn fresh_l(&mut self) -> String {
        let n = self.lc;
        self.lc += 1;
        format!("L{}", n)
    }
    fn fresh_v(&mut self) -> String {
        let n = self.vc;
        self.vc += 1;
        format!("V{}", n)
    }

    fn conv(&mut self, t: &Term, scope: &HashMap<String, String>) -> String {
        match t {
            Term::Var(v) => scope
                .get(v)
                .cloned()
                .unwrap_or_else(|| format!("g_{}", v)),
            Term::App(f, x) => {
                format!("(APPLY {} {})", self.conv(f, scope), self.conv(x, scope))
            }
            Term::Lam(p, body) => {
                let caps = free_vars(t, self.globals); // 捕捉する自由変数（DSL 名）
                let ln = self.fresh_l();
                let pn = self.fresh_v();

                // Lk 内スコープ: 捕捉変数は env から、param は pn。
                let mut s2: HashMap<String, String> = HashMap::new();
                let mut cap_names = Vec::new();
                for v in &caps {
                    let en = self.fresh_v();
                    s2.insert(v.clone(), en.clone());
                    cap_names.push(en);
                }
                s2.insert(p.clone(), pn.clone());
                let body_expr = self.conv(body, &s2);

                let ldef = if cap_names.is_empty() {
                    format!("(define ({} _env {}) {})", ln, pn, body_expr)
                } else {
                    let binds: String = cap_names
                        .iter()
                        .enumerate()
                        .map(|(k, en)| format!("({} (nth {} _env))", en, k))
                        .collect::<Vec<_>>()
                        .join(" ");
                    format!("(define ({} _env {}) (let ({}) {}))", ln, pn, binds, body_expr)
                };
                self.defs.push(ldef);

                // 生成サイトでのクロージャ構築（捕捉値を今のスコープで解決）。
                if caps.is_empty() {
                    format!("(list {})", ln)
                } else {
                    let vals: Vec<String> = caps
                        .iter()
                        .map(|v| {
                            scope
                                .get(v)
                                .cloned()
                                .unwrap_or_else(|| format!("g_{}", v))
                        })
                        .collect();
                    format!("(list {} {})", ln, vals.join(" "))
                }
            }
            Term::HostCall(n, args) => {
                if args.is_empty() {
                    format!("({})", n)
                } else {
                    let a: Vec<String> = args.iter().map(|x| self.conv(x, scope)).collect();
                    format!("({} {})", n, a.join(" "))
                }
            }
            Term::IntLit(n) => n.to_string(),
            Term::StrLit(s) => nl_str(s),
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

    // generate を全面 override するため emit_* は未使用（trait 要件のスタブ）。
    fn emit_lam(&self, _p: &str, _b: &str) -> String {
        unreachable!("newlisp overrides generate (closure conversion)")
    }
    fn emit_app(&self, _f: &str, _x: &str) -> String {
        unreachable!()
    }
    fn emit_host_call(&self, _n: &str, _a: &[String]) -> String {
        unreachable!()
    }
    fn emit_str(&self, s: &str) -> String {
        nl_str(s)
    }
    fn emit_def(&self, _n: &str, _t: &str) -> String {
        unreachable!()
    }
    fn emit_assert(&self, _i: usize, _l: &str, _r: &str) -> String {
        unreachable!()
    }
    fn emit_program(&self, _d: &[String], _a: &[String]) -> String {
        unreachable!()
    }

    fn generate(&self, prog: &Program) -> String {
        let globals: BTreeSet<String> = prog.defs.iter().map(|d| d.name.clone()).collect();
        let mut c = Conv {
            defs: Vec::new(),
            lc: 0,
            vc: 0,
            globals: &globals,
        };
        let empty = HashMap::new();

        let mut gdefs = Vec::new();
        for d in &prog.defs {
            let expr = c.conv(&d.term, &empty);
            gdefs.push(format!("(define g_{} {})", d.name, expr));
        }
        let mut asserts = Vec::new();
        for (i, a) in prog.asserts.iter().enumerate() {
            let l = c.conv(&a.lhs, &empty);
            let r = c.conv(&a.rhs, &empty);
            asserts.push(format!("(_check {} {} \"assert {}\")", l, r, i + 1));
        }

        let mut s = String::new();
        s.push_str("(load \"lam1.lsp\")  ; ヘルパー＋APPLY は lam1.lsp\n");
        s.push_str("\n; --- closures (defunctionalized) ---\n");
        for d in &c.defs {
            s.push_str(d);
            s.push('\n');
        }
        s.push_str("\n; --- definitions ---\n");
        for d in &gdefs {
            s.push_str(d);
            s.push('\n');
        }
        s.push_str("\n; --- assertions ---\n");
        for a in &asserts {
            s.push_str(a);
            s.push('\n');
        }
        s.push_str("\n(_finish)\n");
        s
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
