//! Haskell バックエンド（ML系・遅延）。万能型を初めて要するバックエンド。

use super::Backend;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct Haskell;

impl Backend for Haskell {
    fn name(&self) -> &'static str {
        "haskell"
    }
    fn ext(&self) -> &'static str {
        "hs"
    }
    fn prelude(&self) -> &'static str {
        include_str!("../../../languages/haskell/prelude.hs")
    }

    // Haskell は一律 `_` 前置を維持する（予約語だけでは足りない）:
    //   - 大文字始まりの識別子は値ではなくデータ構築子として解釈される（I/K/S/Z が不正になる）
    //   - `and`/`not`/`succ`/`pred` 等は Prelude と衝突し、トップレベル束縛が曖昧になる
    // `_` 前置はこの両方を一度に回避する（`_I` は小文字始まりの通常の変数）。
    fn mangle(&self, n: &str) -> String {
        format!("_{}", n)
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

    /// `run` 用に build→実行するが、GHC がコンパイル不能な場合は runghc へフォールバックする。
    ///
    /// 自己適用（例 Y/Z コンビネータの `x x`）は GHC の simplifier が `app` の unfold ループに
    /// 陥り、`-O0` でも `Simplifier ticks exhausted` でコンパイルできない。その項に限り、
    /// 最適化器を通さない `runghc`（bytecode）で実行する。標準/bench コーパスは -O2 で通るので
    /// フォールバックせず、実行性能（-O2 の目的）は変わらない。
    fn exec(&self, dir: &Path, file: &Path) -> std::io::Result<bool> {
        let bin = dir.join("main_bin");
        let compiled = Command::new("ghc")
            .args(["-O2", "-o"])
            .arg(&bin)
            .args(["-odir"])
            .arg(dir)
            .args(["-hidir"])
            .arg(dir)
            .arg(file)
            .stderr(Stdio::piped()) // フォールバック時に GHC の大量エラーを出さない
            .stdout(Stdio::null())
            .output()?;
        if compiled.status.success() {
            let st = Command::new(&bin).current_dir(dir).status()?;
            return Ok(st.success());
        }
        eprintln!("(haskell: ghc -O2 でコンパイル不能 — runghc へフォールバック。自己適用か?)");
        let st = Command::new("runghc").arg(file).current_dir(dir).status()?;
        Ok(st.success())
    }
}
