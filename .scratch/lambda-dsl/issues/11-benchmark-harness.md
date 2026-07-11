# 11: ベンチ計測ハーネス（処理速度・メモリ）

Status: ready-for-agent

依存: [08](08-v1-corpus.md)

## 内容

同一のラムダ項を各言語で実行し、**処理速度（wall-clock）とメモリ（最大 RSS）**を横断比較する `bench` サブコマンドを追加する。発表の「対決」表の出処。

## 方針

- **コンパイルは計測外（untimed）**、生成物の**実行だけを計測**する。そのため `Backend` を `build()`（コンパイル。インタプリタ言語は no-op）＋ `run_argv()`（実行コマンド）に分割する。
- 計測は `/usr/bin/time -l`（macOS）で wall-clock（`real`）と `maximum resident set size` を取得。
- 深い再帰（チャーチ数のデコードは値の深さだけスタックを使う）を避けるため、**軽い計算 × 多数の assert** で総処理量を稼ぐベンチ用コーパス（`bench.lam`）を用意。Python は再帰上限を引き上げる。
- 出力は wall-clock 昇順のテーブル。

## 既知の caveat（表に明記する）

- 動的言語はインタプリタ起動込み。JVM 系（Java/Kotlin/Scala/Clojure）は JVM 起動が支配的になりやすい。
- コンパイル系（Haskell=ghc -O2 / Rust=rustc -O / Go / C++=-O2 / SML=MLton / Swift=swiftc -O）は最適化ビルドの実行のみを計測。

## 実装メモ / 追加 caveat

- `Backend` を `build()`＋`run_argv()` に分割。`exec` は両者を合成（`run` 用）。
- ノイズが大きい（単発だとコールドスタートで native が 500ms 台に見える）ため **best-of-3（wall-clock 最小）** を採用。
- **JVM のメソッドは 64KB バイトコード上限**があり、`main` に assert を並べすぎるとコンパイル不能（java/kotlin/scala が n/a になる）。→ assert 数を抑え 1 件あたりの計算量（チャーチ数の大きさ）で総処理量を稼ぐ。
- `/usr/bin/time -l` の `real` は 0.01s 粒度。軽いワークロードでは native 勢が横並び（≤10ms）で解像不能。native 同士を分離するには重いワークロードが要るが JVM メソッド上限とスタック深さがボトルネック。

## 現状の結果（500×mult 30 30、best-of-3、参考値）

native（haskell/rust/go/sml/cpp/swift）≈10ms・1.7〜11MB / java 30・kotlin 40・python 40・perl 40・ruby 50 / php 110 / scala 240 / emacs 260 / clojure 400 / racket 570。数値は「生成コードの実行＝エンコード方式＋処理系起動込み」であり、純粋な言語速度ではない点を発表でも明記する。

## 受け入れ条件

- `translator bench bench.lam` が全 16 言語の wall-clock と最大 RSS を集計・ソート表示する。✅
