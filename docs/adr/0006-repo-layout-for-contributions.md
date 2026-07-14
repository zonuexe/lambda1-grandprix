# 貢献の入口を明示するリポジトリ構成へ再編する

外部から「言語（デモ対象言語）」と「コーパス（DSL ソース `.lam`）」の追加貢献を受け付けるため、手書きソースをトップレベルへ昇格し、生成物・ツールと視覚的に分離する。従来はコーパスとプレリュードが Rust ツールの内部ディレクトリ（`translator/corpus/`・`translator/preludes/`）に埋もれ、生成物ディレクトリ（`code/`）も名前から生成物と分からず、貢献の入口が見えなかった（[[0004-lambda-dsl-and-translator]] のツール構造自体は不変）。

## 主要な決定

- **手書きソースをトップレベルへ昇格**。`translator/corpus/` → `corpus/`（DSL ソース `*.lam`）、`translator/preludes/` → `languages/<言語>/prelude.<ext>`（言語ごとの手書き[[CONTEXT|プレリュード]]）。貢献対象が repo 直下に並ぶ。
- **生成物ディレクトリをドメイン用語へ改名**。`code/` → `demos/`（[[CONTEXT]] の「デモ」）。`translator/scripts/gen-all.sh` が全コーパス × 全言語を `demos/<言語>/<コーパス>/` に生成し、確定版としてコミットする。
- **手書き / 生成物 / ツールの三分割**を構成の第一原則にする。`corpus/`・`languages/` は人が書く入力、`demos/` は再生成可能な出力、`translator/` は変換ツール。README のディレクトリマップでこの区別を明示する。
- **codegen の Rust モジュール構造は温存**。`translator/src/codegen/<言語>.rs` と `mod.rs` の登録方式（[[0004-lambda-dsl-and-translator]] の T3 ハイブリッド）は変えず、プレリュードの `include_str!` 参照先だけを `languages/` へ張り替える。
- **貢献手順を `CONTRIBUTING.md` にチェックリスト化**。「コーパスを追加する」「言語を追加する」の 2 経路を、実際のコード（`Backend` trait・`all_backends()`・`flake.nix`）に沿って文書化する。

## Considered Options

- **配置の粒度**: 手書きソースをトップレベルへ昇格（採用）／ `translator/` 内に留置（現状維持）／ すべてを1つの `src/` に集約。貢献の入口を視覚的に示すことを最優先し、昇格を採用。ツール内留置は「どこを触れば貢献になるか」が伝わらないため不採用。
- **生成物ディレクトリ名**: `demos/`（採用・ドメイン用語）／ `code/`（現状維持）／ `generated/`。ドメイン用語（[[CONTEXT|デモ]]）と一致し、聴衆が「自分の言語のデモ」を探す動線に合う `demos/` を採用。
- **プレリュードの階層**: `languages/<言語>/prelude.<ext>`（採用）／ `preludes/<言語>.<ext>`（フラット）。1言語につき将来 prelude 以外の手書き資産（メモ・追加ヘルパー）が増える余地を残し、言語ごとのディレクトリを採用。
- **codegen 構造の扱い**: 温存（採用）／ この機会に再設計。構成再編とツール再設計を混ぜると生成物の同一性検証が難しくなるため、[[0004-lambda-dsl-and-translator]] の構造は非ゴールとして温存。

## Consequences

- 貢献者は `corpus/` と `languages/` だけを見れば入力の全体像を把握でき、`translator/` の Rust を読まずにコーパスを追加できる。
- 移動・改名の前後で生成物は同一（パスのみ変化）。`translator/scripts/gen-all.sh` を新パスで再実行し、`demos/` が旧 `code/` とパス以外一致することを検証基準とした（PRD の検証基準）。
- 各バックエンドの `prelude()` は `include_str!("../../../languages/<言語>/prelude.<ext>")` を参照する。プレリュードを移動する際はこの相対パスの追従が必要（[[CONTEXT|プレリュード]]の位置が API 契約になる）。
- `demos/` は生成物だがコミットする確定版であり、聴衆が単独で読める（[[CONTEXT|デモ]]の自己完結性）。コーパスや言語を追加したら `demos/` を再生成してコミットする運用が固定される。
- ディレクトリの意味づけ（手書き / 生成物 / ツール）が README・各 `README.md`・`CONTRIBUTING.md` に分散するため、構成を変える際はこれらの同期が必要になる。
