//! PHP バックエンド（動的、arrow fn）。変数は `$_` prefix でマングリング。

use super::Backend;
use std::path::Path;

pub struct Php;

impl Backend for Php {
    fn name(&self) -> &'static str {
        "php"
    }
    fn ext(&self) -> &'static str {
        "php"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/php.php")
    }

    // PHP の変数は `$` 始まり。ホスト関数名（encodeInt 等）には付けない。
    fn mangle(&self, n: &str) -> String {
        format!("$_{}", n)
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("(fn({}) => {})", param, body)
    }
    fn emit_app(&self, f: &str, x: &str) -> String {
        format!("({})({})", f, x)
    }
    fn emit_host_call(&self, name: &str, args: &[String]) -> String {
        format!("{}({})", name, args.join(", "))
    }
    fn emit_str(&self, s: &str) -> String {
        format!("'{}'", s)
    }
    fn emit_def(&self, name: &str, term: &str) -> String {
        format!("{} = {};", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("_check({}, {}, 'assert {}');", lhs, rhs, idx + 1)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let mut s = String::new();
        s.push_str(self.prelude()); // 先頭に <?php を含む
        s.push_str("\n// --- definitions ---\n");
        for d in defs {
            s.push_str(d);
            s.push('\n');
        }
        s.push_str("\n// --- assertions ---\n");
        for a in asserts {
            s.push_str(a);
            s.push('\n');
        }
        s.push_str("\n_finish();\n");
        s
    }

    fn run_argv(&self, _dir: &Path, file: &Path) -> Vec<String> {
        vec!["php".into(), file.to_string_lossy().into_owned()]
    }
}
