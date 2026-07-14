# PRD: 貢献受け入れのためのディレクトリ構成見直し

## 背景 / 課題

外部から「言語（＝新しいデモ対象言語）」と「コーパス（＝DSL ソース `.lam`）」の追加貢献を
受け付けたい。しかし現状の構成では:

- コーパス（`translator/corpus/`）とプレリュード（`translator/preludes/`）が
  Rust ツールの内部ディレクトリに埋まっており、貢献の入口に見えない。
- `code/` は gen-all.sh の生成物（コミットする確定版）だが、名前から生成物と
  分からない。ドメイン用語（CONTEXT.md）では「デモ」。
- 言語追加に必要な作業一覧（codegen モジュール、プレリュード、flake.nix、再生成）が
  どこにも文書化されていない。CONTRIBUTING.md が無い。

## ゴール

1. 貢献対象（コーパス・プレリュード）を手書きソースとしてトップレベルに昇格し、
   生成物・ツールと視覚的に分離する。
2. 生成物ディレクトリをドメイン用語に合わせ `demos/` に改名する。
3. README / CONTRIBUTING で「言語を追加する」「コーパスを追加する」の手順を
   チェックリスト化する。

## 目標構成

```
/
├── README.md            # プロジェクト概要＋ディレクトリマップ＋クイックスタート
├── CONTRIBUTING.md      # 言語追加・コーパス追加の手順
├── CONTEXT.md / AGENTS.md / CLAUDE.md / flake.nix   # 現状維持
├── corpus/              # ← translator/corpus/ から移動（DSL ソース、手書き）
│   └── *.lam
├── languages/           # ← translator/preludes/ から再編（言語ごとの手書き部分）
│   └── <lang>/prelude.<ext>
├── demos/               # ← code/ を改名（gen-all.sh の生成物、コミットする）
│   └── <lang>/<corpus>/{demo.<ext>, lam1.*, source.lam}
├── translator/          # Rust ツール本体（src/, scripts/）
└── docs/                # proposal, adr/, agents/
```

## 非ゴール

- codegen の Rust モジュール構造（translator/src/codegen/）の再設計はしない。
- DSL 文法・生成コードの内容変更はしない（移動・改名前後で生成物は同一）。

## 検証基準

- `cargo build` が通り、`translator/scripts/gen-all.sh` が新パスで動作する。
- 再生成した demos/ の内容が移動前の code/ と（パス以外）一致する。
- ドキュメント内の旧パス参照（code/, translator/corpus, translator/preludes）が残らない。

## Issues

- 01-move-corpus-and-preludes.md — 構造移動＋translator のパス追従
- 02-rename-code-to-demos.md — 生成物ディレクトリ改名＋再生成
- 03-docs-and-contributing.md — README/CONTRIBUTING/ADR/CONTEXT 更新
