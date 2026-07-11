# 12: Lazy K バックエンド（bracket abstraction ＋ インライン）

Status: ready-for-agent

依存: [02](02-codegen-core.md)

## 背景

別セッションで実装中の Lazy K インタプリタ（独立クレート）向けの**テストデータ**を生成する。Lazy K は純粋 SKI コンビネータ計算で、λ抽象も名前付き定義もホスト整数/文字列/IO プリミティブも無い。

## 方針（決定済み）

- **λ の除去**: bracket abstraction（`T[λx.x]=I` / `T[λx.M]=K T[M]`（x∉M）/ `T[λx.MN]=S T[λx.M] T[λx.N]`）。
- **名前の除去**: 各定義は bracket abstraction 後に**閉じたコンビネータ項**になるので、参照箇所へ**インライン展開**（閉じているので捕獲は起きない）。
- `encodeInt[N]` → チャーチ数 N のコンビネータ（`λf.λx.f^N x` を abstraction）。`decodeInt`/`decodeBool` はホスト境界で Lazy K に対応物が無いため、ラッパを外して内側の項だけを出す。
- **出力**: SKIスタイル（`S`/`K`/`I`＋並置＋`( )`）の**純粋なコンビネータ項のみ**（IO なし）。各定義・各 assert の内側計算を、コメントラベル付きで1項ずつ出力。
- **実行はしない**（Lazy K インタプリタは本環境に無い）。生成した静的ファイルが成果物。

## 注意

- bracket abstraction もインラインもサイズが爆発する（共有機構が無い）。良いストレステストデータになる。

## 受け入れ条件

- `translator run --lang lazyk <file>` が `.gen/lazyk/main.lazy` に SKI 項を生成する。
- `S K K (encodeInt[1])` が `S K K (church1)` 相当の SKI に、`I`/`K`/`S` が自身に落ちること。
