//! Free Pascal バックエンド（FPC 3.2.2・無名関数なし → クロージャ変換）。
//!
//! 各 λ を「捕捉した自由変数をフィールドに持ち、`Apply` メソッドで本体を実行する
//! クラス」へ変換する。ネストした λ は `TCloN.CreateC(捕捉値...)` として構築する。
//! 万能型 TValue はプレリュード側（TNum/TStr ＋ クロージャオブジェクト）。

use super::Backend;
use crate::ast::{Program, Term};
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

pub struct Pascal;

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

/// クロージャ変換の作業状態。生成したクラス宣言・実装を貯める。
struct Conv {
    decls: Vec<String>,
    impls: Vec<String>,
    counter: usize,
    globals: BTreeSet<String>,
}

impl Conv {
    /// 現在フレーム（param＝Apply の引数、fields＝捕捉フィールド、それ以外はグローバル）
    /// における `v` の Pascal 式。
    fn resolve(&self, v: &str, param: Option<&str>, fields: &BTreeSet<String>) -> String {
        if param == Some(v) {
            "arg".to_string()
        } else if fields.contains(v) {
            format!("f_{}", v)
        } else if self.globals.contains(v) {
            format!("_{}", v)
        } else {
            format!("{{unbound:{}}}", v)
        }
    }

    fn conv(&mut self, t: &Term, param: Option<&str>, fields: &BTreeSet<String>) -> String {
        match t {
            Term::Var(v) => self.resolve(v, param, fields),
            Term::App(f, x) => {
                let cf = self.conv(f, param, fields);
                let cx = self.conv(x, param, fields);
                format!("{}.Apply({})", cf, cx)
            }
            Term::Lam(p, body) => {
                // 捕捉するのは「グローバルでない自由変数」（グローバルは直接参照）。
                let captured: Vec<String> = free_vars(t)
                    .into_iter()
                    .filter(|v| !self.globals.contains(v))
                    .collect();
                let idx = self.counter;
                self.counter += 1;
                let cls = format!("TClo{}", idx);
                let field_set: BTreeSet<String> = captured.iter().cloned().collect();
                let body_expr = self.conv(body, Some(p), &field_set);

                // クラス宣言
                let mut decl = format!("  {} = class(TValue)\n  public\n", cls);
                for v in &captured {
                    decl.push_str(&format!("    f_{}: TValue;\n", v));
                }
                if !captured.is_empty() {
                    let ps: Vec<String> =
                        captured.iter().map(|v| format!("p_{}: TValue", v)).collect();
                    decl.push_str(&format!("    constructor CreateC({});\n", ps.join("; ")));
                }
                decl.push_str("    function Apply(arg: TValue): TValue; override;\n  end;\n");
                self.decls.push(decl);

                // クラス実装
                let mut imp = String::new();
                if !captured.is_empty() {
                    let ps: Vec<String> =
                        captured.iter().map(|v| format!("p_{}: TValue", v)).collect();
                    imp.push_str(&format!("constructor {}.CreateC({});\nbegin\n", cls, ps.join("; ")));
                    for v in &captured {
                        imp.push_str(&format!("  f_{} := p_{};\n", v, v));
                    }
                    imp.push_str("end;\n\n");
                }
                imp.push_str(&format!(
                    "function {}.Apply(arg: TValue): TValue;\nbegin\n  Result := {};\nend;\n\n",
                    cls, body_expr
                ));
                self.impls.push(imp);

                // 現在フレームでの式（捕捉値を今の文脈で解決して Create）
                if captured.is_empty() {
                    format!("{}.Create", cls)
                } else {
                    let args: Vec<String> = captured
                        .iter()
                        .map(|v| self.resolve(v, param, fields))
                        .collect();
                    format!("{}.CreateC({})", cls, args.join(", "))
                }
            }
            Term::HostCall(n, args) => {
                let a: Vec<String> = args.iter().map(|x| self.conv(x, param, fields)).collect();
                format!("{}({})", n, a.join(", "))
            }
            Term::IntLit(n) => n.to_string(),
            Term::StrLit(s) => format!("'{}'", s),
        }
    }
}

impl Backend for Pascal {
    fn name(&self) -> &'static str {
        "pascal"
    }
    fn ext(&self) -> &'static str {
        "pas"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../../languages/pascal/prelude.pas")
    }

    // generate を全面 override するため emit_* は未使用（trait 要件のスタブ）。
    fn emit_lam(&self, _p: &str, _b: &str) -> String {
        unreachable!("pascal overrides generate")
    }
    fn emit_app(&self, _f: &str, _x: &str) -> String {
        unreachable!()
    }
    fn emit_host_call(&self, _n: &str, _a: &[String]) -> String {
        unreachable!()
    }
    fn emit_str(&self, _s: &str) -> String {
        unreachable!()
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
            decls: Vec::new(),
            impls: Vec::new(),
            counter: 0,
            globals,
        };
        let empty = BTreeSet::new();

        let mut global_decls = String::new();
        let mut inits = String::new();
        for d in &prog.defs {
            let expr = c.conv(&d.term, None, &empty);
            global_decls.push_str(&format!("  _{}: TValue;\n", d.name));
            inits.push_str(&format!("  _{} := {};\n", d.name, expr));
        }

        let mut asserts = String::new();
        for (i, a) in prog.asserts.iter().enumerate() {
            let l = c.conv(&a.lhs, None, &empty);
            let r = c.conv(&a.rhs, None, &empty);
            asserts.push_str(&format!("  check('assert {}', {}, {});\n", i + 1, l, r));
        }

        self.prelude()
            .replace("{__CLASS_DECLS__}", &c.decls.join(""))
            .replace("{__GLOBALS__}", &global_decls)
            .replace("{__CLASS_IMPLS__}", &c.impls.join(""))
            .replace("{__INIT__}", &inits)
            .replace("{__ASSERTS__}", &asserts)
    }

    fn build(&self, dir: &Path, _file: &Path) -> std::io::Result<bool> {
        // FPC 3.2.2 の .o は新しい macOS リンカ(ld64)が alignment を拒否するため、
        // 旧リンカ経路 `-ld_classic` を使う（-k で ld へ渡す）。
        let st = Command::new("fpc")
            .current_dir(dir)
            .arg("-k-ld_classic")
            .arg("-omain_bin")
            .arg("main.pas")
            .status()?;
        Ok(st.success())
    }
    fn run_argv(&self, dir: &Path, _file: &Path) -> Vec<String> {
        vec![dir.join("main_bin").to_string_lossy().into_owned()]
    }
}
