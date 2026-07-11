//! Haskell バックエンド（ML系・遅延）。万能型を初めて要するバックエンド。

use super::Backend;
use std::path::Path;
use std::process::Command;

pub struct Haskell;

impl Backend for Haskell {
    fn name(&self) -> &'static str {
        "haskell"
    }
    fn ext(&self) -> &'static str {
        "hs"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../preludes/haskell.hs")
    }

    fn emit_lam(&self, param: &str, body: &str) -> String {
        format!("(Fun (\\{} -> {}))", param, body)
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
        format!("{} = {}", name, term)
    }
    fn emit_assert(&self, idx: usize, lhs: &str, rhs: &str) -> String {
        format!("check \"assert {}\" ({}) ({})", idx + 1, lhs, rhs)
    }
    fn emit_program(&self, defs: &[String], asserts: &[String]) -> String {
        let mut s = String::new();
        s.push_str(self.prelude());
        s.push_str("\n-- --- definitions ---\n");
        for d in defs {
            s.push_str(d);
            s.push('\n');
        }
        s.push_str("\n-- --- main ---\nmain :: IO ()\nmain = do\n");
        s.push_str("  results <- sequence\n    [ ");
        s.push_str(&asserts.join("\n    , "));
        s.push_str("\n    ]\n");
        s.push_str("  let fails = length (filter not results)\n");
        s.push_str("  if fails > 0 then putStrLn (show fails ++ \" failure(s)\") >> exitFailure else putStrLn \"all green\"\n");
        s
    }

    fn build(&self, dir: &Path, file: &Path) -> std::io::Result<bool> {
        let bin = dir.join("main_bin");
        let st = Command::new("ghc")
            .arg("-O2")
            .arg("-o")
            .arg(&bin)
            .arg("-odir")
            .arg(dir)
            .arg("-hidir")
            .arg(dir)
            .arg(file)
            .status()?;
        Ok(st.success())
    }
    fn run_argv(&self, dir: &Path, _file: &Path) -> Vec<String> {
        vec![dir.join("main_bin").to_string_lossy().into_owned()]
    }
}
