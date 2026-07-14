# λ-1グランプリ

型なしラムダ計算を、複数のプログラミング言語（処理系）で実装・比較する発表用の実コード置き場です。「ラムダ抽象」と「適用」というシンプルな規則だけでプログラミングの基本構造やデータ表現がどう書けるか、そして現実の言語でラムダ式（関数式・関数抽象）がどう実装されているかを、同じ題材で横並びに比較します。詳細は [`docs/proposal.md`](docs/proposal.md) を参照してください。

中核は、型なしラムダ計算を記述する独自の **DSL**（名前付き定義 `I = λx.x` と `assert` からなる）と、それを各言語のネイティブなクロージャへ**直接翻訳**する Rust 製の**トランスレータ**です。ラムダ世界の値はすべて純チャーチエンコード（＝関数）で表し、ホスト言語の整数・真偽値などとは encode/decode の**境界**でのみ相互変換します。用語の定義は [`CONTEXT.md`](CONTEXT.md)、設計の決定記録は [`docs/adr/`](docs/adr/) にあります。

## ディレクトリマップ

手書きソース（貢献の入口）・生成物・ツールを分けています。

| パス | 種別 | 内容 |
| --- | --- | --- |
| `corpus/` | 手書き | DSL ソース `*.lam`。各言語デモの単一の出処。 |
| `languages/<言語>/prelude.<ext>` | 手書き | 言語ごとの[境界ヘルパー](CONTEXT.md)（`encode*`/`decode*`/`myassert`/万能型）。 |
| `demos/` | **生成物** | `gen-all.sh` の出力。`demos/<言語>/<コーパス>/` に自己完結配置（`demo.<ext>` ＋ 分離ライブラリ ＋ `source.lam`）。コミットする確定版。 |
| `translator/` | ツール | Rust 製トランスレータ本体（`src/`）と生成スクリプト（`scripts/`）。 |
| `docs/` | ドキュメント | `proposal.md`, ADR（`adr/`）, エージェント運用（`agents/`）。 |
| `CONTEXT.md` | ドキュメント | ドメイン用語集。 |

`demos/` は自動生成物です。手で編集せず、`corpus/` か `languages/` を変えてから再生成してください。

## クイックスタート

処理系のツールチェーンは Nix flake で固定しています（[ADR-0001](docs/adr/0001-nix-flake-for-toolchains.md)）。

```sh
# 1. 開発シェルに入る（各言語の処理系・Rust が揃う）
nix develop        # direnv 派は `direnv allow` でも可

# 2. 全コーパス × 全言語のデモを demos/ に生成
translator/scripts/gen-all.sh

# 3. 特定の言語・コーパスのデモを実行（例: Python の v1 コーパス）
translator/scripts/gen-all.sh   # 生成済みなら不要
cd demos/python/v1 && python3 demo.py
```

トランスレータ単体でも扱えます（生成のみ・実行含む・ベンチ）。

```sh
cd translator
cargo run -- gen  --lang python  ../corpus/v1.lam           # 読める成果物を出力（実行しない）
cargo run -- run  --lang haskell ../corpus/recursion.lam     # 生成して assert を実行・確認
cargo run -- bench             ../corpus/bench.lam           # 実行時間・最大 RSS を計測
```

`--lang` を省くと全言語が対象になります。生成物ディレクトリ（`gen-all.sh` の `demos/`）は、聴衆が自分の言語のデモだけを単独で読み・動かせるよう自己完結しています。

## 貢献

新しいコーパス（`.lam`）や言語（デモ対象）の追加を歓迎します。手順は [`CONTRIBUTING.md`](CONTRIBUTING.md) を参照してください。
