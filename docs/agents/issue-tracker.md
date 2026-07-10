# Issue トラッカー: ローカル Markdown

このリポジトリの issue と PRD は `.scratch/` 配下の markdown ファイルとして管理します。

## 規約

- 1機能につき1ディレクトリ: `.scratch/<機能スラッグ>/`
- PRD は `.scratch/<機能スラッグ>/PRD.md`
- 実装 issue は `.scratch/<機能スラッグ>/issues/<NN>-<スラッグ>.md`（`01` から連番）
- トリアージの状態は各 issue ファイル冒頭付近の `Status:` 行に記録します（役割の文字列は `triage-labels.md` を参照）
- コメントや会話履歴はファイル末尾の `## Comments` 見出しの下に追記します

## skill が「issue トラッカーに publish する」と言った場合

`.scratch/<機能スラッグ>/` 配下に新しいファイルを作成します（必要ならディレクトリも作成）。

## skill が「該当チケットを fetch する」と言った場合

参照されたパスのファイルを読みます。通常はユーザーがパスまたは issue 番号を直接指定します。
