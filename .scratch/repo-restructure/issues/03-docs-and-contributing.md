# 03: README / CONTRIBUTING / ADR / CONTEXT の整備

Status: ready-for-human

## 作業内容

- `README.md` 拡充: プロジェクト概要、ディレクトリマップ（手書き vs 生成物の区別を明示）、
  クイックスタート（nix develop → gen-all.sh → デモ実行）
- `CONTRIBUTING.md` 新規: 「コーパスを追加する」（corpus/*.lam を書く→再生成）と
  「言語を追加する」（languages/<lang>/prelude、translator/src/codegen/<lang>.rs、
  mod.rs 登録、flake.nix、再生成、動作確認）のチェックリスト
- `docs/adr/0006-repo-layout-for-contributions.md` 新規: 本再編の決定記録
  （既存 ADR の体裁に合わせる: 決定・Considered Options・Consequences）
- `CONTEXT.md` / `docs/adr/0004` 等、旧パス（code/, translator/corpus, translator/preludes）
  への言及を新パスへ更新
- `corpus/README.md` と `languages/README.md` に短い案内（何を置く場所か、詳細は
  CONTRIBUTING 参照）

## 依存

- 01, 02 完了後

## 受け入れ条件

- `rg 'code/|translator/corpus|translator/preludes' docs *.md` で旧パス参照ゼロ
  （ADR の過去の決定記述として意図的に残すものは除く）
- CONTRIBUTING の手順どおりに新言語追加が完結することが読んで分かる

## Comments

### 実施内容（2026-07-14）

- `README.md` を拡充: プロジェクト概要（proposal.md/CONTEXT.md を要約）、ディレクトリマップ
  （手書き `corpus/`・`languages/` vs 生成物 `demos/` vs ツール `translator/` を表で明示）、
  クイックスタート（nix develop → gen-all.sh → デモ実行、cargo run の run/bench/gen も併記）、
  貢献は CONTRIBUTING.md 参照の一文。
- `CONTRIBUTING.md` を新規作成: 「コーパスを追加する」「言語を追加する」のチェックリスト。
  言語追加は実コードに沿って記述（`Backend` trait の必須/任意メソッド、`prelude()` の
  `include_str!("../../../languages/<lang>/prelude.<ext>")`、`mod.rs` の `pub mod` 宣言 ＋
  `all_backends()` の `vec!` 登録、ハイフン名→アンダースコアのモジュール規約、flake.nix の
  packages/shellHook、再生成・動作確認）。想像でなく src/codegen/{mod.rs,python.rs,haskell.rs 他}・
  main.rs・gen-all.sh・flake.nix を読んで確認済み。
- `docs/adr/0006-repo-layout-for-contributions.md` を新規作成: 既存 ADR の体裁
  （H1 決定文＋導入＋主要な決定＋Considered Options＋Consequences、[[wikilink]]）に合わせた。
- `corpus/README.md`・`languages/README.md` を新規作成（数行＋CONTRIBUTING 参照）。
- 旧パス参照の更新: 洗い出しの結果、docs/*.md・CONTEXT.md・.envrc・docs/agents/*.md に
  更新すべき旧パス参照は**残っていなかった**（issue 01/02 で追従済み）。`rg 'code/|...'` の
  ヒットは全て `encode/decode`（"code/" を含む）の誤検出、および ADR-0006 内の移行記録として
  意図的に残す歴史記述のみ。ADR-0004 も該当行は `encode/decode` の誤検出でパス参照ではない。

補足: `translator/preludes/` が空の untracked ディレクトリとして残っているが、git 管理外・
無害のため未削除（本 issue の対象外）。`demos/` は変更せず、コミットもしていない。
