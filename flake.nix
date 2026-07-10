{
  description = "λ-1グランプリ: 型なしラムダ計算を複数言語で実装・比較する処理系一括環境";

  # 方針は docs/adr/0001-nix-flake-for-toolchains.md を参照。
  # - 安定リリースを入力に固定（B2）。版は flake.lock で一意に固定する。
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
  };

  outputs = { self, nixpkgs }:
    let
      # 主対象は登壇者の Apple Silicon Mac。将来 Linux でベンチ比較する余地を残す。
      systems = [ "aarch64-darwin" "x86_64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };

          # JVM 系 4 言語（Java / Kotlin / Scala / Clojure）は単一 JDK を共有する（C1）。
          jdk = pkgs.jdk21;
        in
        {
          default = pkgs.mkShell {
            name = "lambda1-grandprix";

            # Swift と C/C++ はここに入れない。Xcode 付属の
            # /usr/bin/swift, /usr/bin/clang++ をそのまま使う（ADR-0001）。
            # Lazy K は本リポジトリの Rust 実装（rustc/cargo で build）。
            packages = with pkgs; [
              # JVM 系（共有 JDK 21）
              jdk
              clojure
              rlwrap # clojure の clj 用
              kotlin
              scala_3

              # スクリプト / 汎用言語
              perl
              php
              ruby
              python3

              # ネイティブ / コンパイラ系
              go
              rustc
              cargo # Rust ＋ Lazy K 自作実装のビルドに使用
              ghc # Haskell
              mlton # Standard ML（SML# の代替。ADR-0002）
              fpc # Free Pascal

              # Lisp / Scheme
              emacs-nox # Emacs Lisp（emacs --batch で実行。GUI 不要）
              racket
            ];

            shellHook = ''
              export JAVA_HOME="${jdk}"
              echo "λ-1グランプリ devShell（Nix flake / nixos-26.05 固定）"
              echo "  Nix 管理: Emacs Lisp, Racket, Clojure, Perl, Free Pascal, Java,"
              echo "            PHP, Kotlin, Scala, Ruby, Haskell(GHC), Standard ML(MLton),"
              echo "            Python, Go, Rust"
              echo "  システム: Swift(/usr/bin/swift), C++(/usr/bin/clang++)  ※Xcode 付属"
              echo "  Lazy K  : 本リポジトリの Rust 実装"
            '';
          };
        });
    };
}
