# Lazy K は外部処理系に依存せず Rust でスクラッチ実装する

Lazy K は Homebrew にも nixpkgs にも存在しない esoteric 言語で、外部処理系として一括管理に載せられない。仕様が小さいため、外部依存にせず **Rust でスクラッチ実装し、リポジトリ自身の成果物**とする。これにより Rust ツールチェーン1本（既に Nix flake 管理下）に集約でき、かつ「小さなラムダ計算ベースの言語を自作する」という発表の主題（`docs/proposal.md`）とも噛み合う。

なお、この自作 Lazy K 実装は他言語の「デモ」と異なり、それ自体が発表対象の処理系である点に注意（[[CONTEXT]] の「処理系」参照）。

## 改訂（2026-07）: `zonu-lazyk` crate 化と translator 連携

上記スクラッチ実装は **`zonu-lazyk` crate**（crates.io 公開）として切り出した。決定自体（外部処理系に依存せず Rust 実装・リポジトリの成果物）は不変で、配布形態を crate にした改訂である。translator は本 crate を薄くラップした同梱ランナー bin（`lazyk-runner`）経由で **実行・検証**する（従来は SKI 項を生成するのみで未実行だった）。ランナー方式は他バックエンドの「生成 → 外部プロセス実行」モデルと揃い、bench 計測もそのまま使える（[[0004-lambda-dsl-and-translator]]）。

## 入出力インターフェイス（現状）

`zonu_lazyk::run(src: &str, input: impl Read + 'static, output: impl Write) -> Result<(), Error>`。

出力プロトコルは Lazy K 標準そのもの: プログラムを input に適用した結果を **`λf. f head tail` リスト**として読む。`list K` が head（チャーチ数）、`list (K I)` が tail。head は `head Inc 0` で強制して整数化し、**値 ≥ 256 を EOF** として各要素を 1 バイト（u8）で書き出す。input は Read から遅延生成されるチャーチ数リスト。

このバイトストリーム I/O は Lazy K として正しいが、**λ-1 の観測（decode）用途には粒度が粗い**。

## λ-1 フォーマットとのギャップ

λ-1 の assert は「decode の**結果値**」を突き合わせる（[[0005-typed-values-and-json]]、[[CONTEXT|境界]]）。byte-stream I/O しか無いため、現在の translator は次の迂回を強いられている:

- **decodeInt**（チャーチ数 → 整数）: 数値を直接取り出す口が無いため、`N (cons b) end` で **N バイト出力させ、その長さを数える**単項カウントに落としている。値が大きいと出力も比例して膨らむ（例 900 → 900 バイト）。
- **decodeBool**（チャーチ真偽値 → true/false）: 真偽値に `"true"`/`"false"` のバイト列を**選ばせて**出力し、文字列比較。動くが間接的。
- **decodeJson**（Scott リスト等の構造値）: 構造を保ったまま観測する口が無く、純 SKI で JSON 文字列へ落とすのも非現実的なため **非対応（skip）**。

## 望ましい性質（wiring / I/O を λ-1 フォーマットに寄せるため）

`zonu-lazyk` が次を公開すると上記の迂回が不要になり、より多くの decode を素直に検証できる。優先度順:

1. **項をチャーチ数へ評価する API**（値の直接観測）
   閉じた項を WHNF まで簡約し、チャーチ数を host 整数（`u64` 等、256 上限なし）として取り出す公開関数（現状 `io::numeral_value` 相当が private）。→ decodeInt の「N バイト出して数える」迂回を除去し、大きな値も O(1) で観測。

2. **`λf. f h t` リストの走査ヘルパー**（構造の観測）
   出力を u8 ストリームに潰さず、リストを head/tail で辿り**各要素を項（または数値）として**取り出す反復子。→ [[0005-typed-values-and-json]] の Scott リスト（string=バイト列 / array / object）をホスト側で再帰 decode でき、decodeJson への道が開ける。

3. **結果を潰さず受け取る出力形**
   EOF センチネル（≥ 256）を**設定可能 or 無効化**でき、各要素を u8 に切り詰めず**生のチャーチ数値の列**として受け取れる形。→ 我々の EOF 規約とデータの衝突を避け、バイト以外の数値列も扱える。既定は変えず**オプトインの追加口**とする（後方互換）。

4. **項を直接与える入口**（render → 再パースの往復回避）
   `term::Comb`（または AST）を直接 `run`/`eval` に渡せる公開経路。translator は既に SKI 項を構築しているので、文字列へ render → 再 parse する往復を省ける。parse / compile / eval を分けて公開する（現状ある程度分離済み）。

5. **有界評価（ステップ/時間上限）**
   非停止項に対する上限と、それを表す固有の `Error` バリアント（niilisp の `set_eval_limit` 相当）。→ DSL から来る任意項を検証する際の安全弁。

6. **入力の与えやすさ／数値変換ヘルパー**
   `&[u8]` / `&str` を `'static` 制約なしに渡せる薄いヘルパーと、host 整数 ↔ チャーチ数リストの相互変換の公開。→ encode/decode 境界を first-class に。

7. **埋め込み安全な失敗**（プロセス終了 / panic ではなく `Result<_, Error>`）。現状ほぼ満たす。ランナーはこれをプロセス終了コードへ翻訳する。

## Consequences

- 当面は **byte-stream I/O のまま**、raw 層（int = 出力長、bool = 文字列選択）を検証し、JSON 層は skip する（現状の translator 実装）。性質 1・2 が入り次第、decodeInt を直接観測へ移し、性質 2 で string/array の段階的な decode を有効化する。
- EOF =「チャーチ数 ≥ 256」の規約は Lazy K 標準として維持。性質 3 は既定を変えない追加口として設ける。
- これらは `zonu-lazyk` 側の公開 API 拡張であり、[[CONTEXT|デモ]]（生成物）のフォーマットや Lazy K 言語仕様には影響しない。
