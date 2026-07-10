# 処理系の一括管理は単一の Nix flake で行う

複数言語の処理系を一括管理するにあたり、mise / Homebrew / Linux コンテナではなく **単一の Nix flake** を採用する。理由は、(1) 発表で処理速度・メモリを数値提示するため `flake.lock` で全処理系のバージョンを厳密固定して数字を版に紐付けたい、(2) mise の PHP/Perl/Racket 等の source ビルドの遅さ・脆さを nix のバイナリキャッシュで回避できる、(3) 単一ツール・単一 lockfile で心的モデルが1つで済む、の3点。

## Considered Options

- **mise ＋ Homebrew**（P0）— 最も手軽だが brew が HEAD 追従でバージョン非固定。ベンチ数値の再現に不利。
- **mise 中心 ＋ Nix/brew の尾**（P1）— mise を主流言語に、変則言語を Nix/brew に。2系統・2 lockfile の心的コスト。
- **全部 Nix flake**（P2, 採用）
- **Debian コンテナ（apple-container/docker）**（P3）— 聴衆が18言語を丸ごと再現する要件が無いため過大。当初は darwin のニッチビルド回避が利点だったが、下記 ADR-0002/0003 で難物を解消したため不要に。

## Consequences

- **Swift と C/C++ は Nix で管理せず Xcode 付属（`/usr/bin/swift`, `clang`）を使う。** darwin の nix swift はビルドが茨で、システム版が最速かつ確実なため。
- 発表はライブ実演せず「実装結果（処理速度・メモリ）とコードのスクリーンショット」のみ提示する。したがって REPL や GUI は要件ではなく、バッチ実行できれば足りる。
- 各言語のデモは自己完結（[[CONTEXT]] の「デモ」参照）で、登壇者マシンでの一括ビルド・計測が目的。聴衆は自分の1言語だけを各自の環境で試す。
