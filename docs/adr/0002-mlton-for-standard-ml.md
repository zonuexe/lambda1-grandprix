# Standard ML は SML# ではなく MLton を使う

`docs/proposal.md` 由来の当初案では Standard ML の処理系に SML# を想定していたが、SML# は Homebrew にも nixpkgs（aarch64-darwin）にも無く、darwin/arm64 での source ビルドが困難。代替として **MLton** を採用する。MLton は whole-program 最適化のネイティブコンパイラでランタイム性能が最良であり、Homebrew・nixpkgs 双方で提供されている。

MLton は REPL を持たないが、本発表はライブ実演せず処理速度・メモリを計測して提示する趣旨（[[0001-nix-flake-for-toolchains]]）のため対話環境は不要で、むしろ最速の処理系である点が計測目的に合致する。対話実演が必要になった場合の次点は Poly/ML（REPL あり・十分高速）。
