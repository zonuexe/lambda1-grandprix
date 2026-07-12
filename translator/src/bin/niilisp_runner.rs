//! niiLISP ランナー。生成された .lsp を niilisp crate で in-process 評価する。
//!
//! 従来は外部 `newlisp` バイナリへ shell していたが、niiLISP の実処理系（crate）で
//! 直接動かす。プログラム末尾の `(exit N)` は `Signal::Exit(N)` として返るので、
//! それをそのままプロセス終了コードに反映する（全 assert 緑なら 0）。

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: niilisp-runner <file.lsp>");
            std::process::exit(2);
        }
    };
    let src = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("niilisp-runner: cannot read {path}: {e}");
        std::process::exit(2);
    });

    let interp = niilisp::Interp::new();
    match interp.eval_string(&src) {
        Ok(_) => {}                                        // exit を呼ばず末尾到達
        Err(niilisp::Signal::Exit(code)) => std::process::exit(code),
        Err(e) => {
            eprintln!("niilisp-runner: {e:?}");
            std::process::exit(1);
        }
    }
}
