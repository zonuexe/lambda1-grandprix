# 14: newlisp/niilisp — PRED 深入れ子で `expand` 焼き込みの V 名衝突

Status: needs-triage

依存: [09](09-more-backends.md)、[13](13-json-layer.md)
対象: `translator/src/codegen/newlisp.rs` ＋ `translator/preludes/newlisp.lsp`
関連コーパス: `corpus/json.lam`（`range`）

## 現象

`translator run --lang newlisp corpus/json.lam` で **assert 1–9 は緑、10–11 が失敗**する。

| assert | 内容 | 結果 |
| -- | -- | -- |
| 9  | `[]`      = `RANGE (encodeInt 0)` | ✅ |
| 10 | `[1]`     = `RANGE (encodeInt 1)` | ❌ |
| 11 | `[1,2,3]` = `RANGE (encodeInt 3)` | ❌ |

- 実 niiLISP（`niilisp` crate / 同梱 `niilisp-runner`）: `Error("parameter is not a symbol: true")`
- 外部 newLISP: `invalid function : (V2 (lambda (V17) …))`
- **両処理系で同じ失敗** → インタプリタではなく backend の符号化（codegen）の問題。
- `range` を使わない項（`corpus/v1.lam`、`corpus/bench.lam`）は全て緑。
- assert 9 が通るのは n=0 だと `STEP` が 0 回適用され **`PRED` が一度も簡約されない**ため。失敗の引き金は **`PRED`（チャーチ前者関数）を `range` の反復内で深く入れ子に簡約する**こと（n ≥ 1）。

## 根本原因

newlisp backend は「ダイナミックスコープ上で擬似レキシカルクロージャ」を作るため、`define-macro (LAMBDA)` が生成時に `expand` で**現在束縛されている大文字シンボルを本体へ焼き込む**（kosh04 方式）。束縛変数は `render` override で `V0, V1, …` の**大域一意名**へα変換している。

問題は **`mangle` が DSL の大域定義名も一律 UPPERCASE にしている**こと（`translator/src/codegen/newlisp.rs` の `fn mangle → to_uppercase`）。その結果:

- 大域定義（`PAIR` `CONS` `PRED` `TINT` `STEP` …）も、束縛ローカル（`V0`, `V1`, …）も、どちらも大文字 → **`expand` が両方を焼き込み対象にする**。
- `expand` は大域を「名前参照」ではなく**本体（V 名を含む lambda）を丸ごとインライン複製**して焼き込む。例えば `STEP` の内側 lambda が `PAIR`/`PRED`/`CONS`/`TINT` を参照すると、それぞれの本体がコピーされ、`PRED` 内の `V16..V22` 等が **STEP のコピー内に重複**する。

```lisp
; PRED の本体には V16..V22 が含まれる
(define PRED (LAMBDA (V16) (LAMBDA (V17) (LAMBDA (V18)
  (((V16 (LAMBDA (V19) (LAMBDA (V20) (V20 (V19 V17)))))
    (LAMBDA (V21) V18)) (LAMBDA (V22) V22))))))
; STEP は PAIR/PRED/CONS/TINT を参照 → expand がそれらの本体を STEP へ複製
(define STEP (LAMBDA (V24) (V24 (LAMBDA (V25) (LAMBDA (V26)
  ((PAIR (PRED V25)) ((CONS (TINT V25)) V26)))))))
```

`range` は church 数 n が `STEP` を n 回適用し、その各段で `PRED` 自身も反復簡約する。α変換は**静的なソース**上では V 名を一意にしているが、`expand` のインライン複製が **同じ V 名のコピーを実行時に複数生成**する。ダイナミックスコープ下では、異なる動的深さにある同名 `V…` が別化されずエイリアスし、**束縛位置（lambda の仮引数）に記号ではなく値（`true` やラムダ）が入り込む** → `parameter is not a symbol` / `invalid function`。

要するに **「静的 α変換の一意性が、`expand` の実行時インライン複製で破られる」**。

## 望ましい振る舞い

- `RANGE (encodeInt n)` が `[1..n]` を返し、`corpus/json.lam` が **11/11 緑**。他の全 backend（[13](13-json-layer.md) で緑化済み）と一致する。
- 一般化: チャーチ前者関数や深い自己適用を含む**任意の well-typed な λ 項**が正しく評価できる（PRED を含まない項に限定されない）。

## 期待する改修

大域定義は**トップレベル `(define …)` で安定**しており、ダイナミックな文脈に依らず名前参照で解決できる。大域が捕捉する自由変数は**他の大域だけ**（DSL の定義項は閉じている）＝ローカルを捕捉しないので、**大域は焼き込む必要が一切ない**。焼き込みが本当に要るのは、外側 λ の仮引数を内側 λ が捕捉する**ローカル自由変数（V 名）だけ**。

**推奨（approach A: 大域を焼き込み対象から外す）**
- DSL 大域定義名を **`expand` が焼き込まない名前空間**で参照する（単一トップレベル定義のまま・本体を複製しない）。焼き込みは**捕捉ローカル（V 名）のみ**に限定する。
- 制約: 大域名は (1) `expand` に焼き込まれない、(2) 小文字 newLISP/niiLISP 組込みと衝突しない、の両方を満たす命名が必要（現状の UPPERCASE は (2) の衝突回避が目的だった）。例: `g_` 等の接頭辞や、組込みと重ならない専用プレフィックス。`expand` の焼き込み判定（「大文字シンボル」の厳密な条件）を確認して名前空間を分ける。
- 参考: JSON 層プレリュード（`preludes/newlisp.lsp`）で同種の衝突を解いた際、**捕捉変数=一意大文字 / 非捕捉=小文字（expand に焼かれない）** の切り分けで通した実績がある。同じ原則を codegen（`mangle`/`render_scoped`）へ適用する。

**代替**
- approach B: `expand` の各展開サイトで焼き込む本体の V 名を gensym で改名（真の per-site α変換）。`expand` は実行時操作なので、`LAMBDA` マクロ側で焼き込み本体を rename する必要があり実装は重い。
- approach C: `expand` 焼き込み方式自体をやめ、明示的な環境渡し（defunctionalization）でレキシカルクロージャを表現する作り替え。最も大きい。

## 受け入れ条件

- `translator run --lang newlisp corpus/json.lam` が **11/11 緑**（実 niiLISP = `niilisp-runner` で確認）。
- `corpus/v1.lam` / `corpus/bench.lam` は緑のまま（回帰なし）。
- 可能なら外部 newLISP（`LAM1_NEWLISP=newlisp`）でも同結果を確認。

## Comments

- 2026-07: crate 統合（`niilisp` 0.4）で実 niiLISP 上でも再現を確認。外部 newLISP と同一の失敗であり、backend codegen の問題と確定。JSON 層自体（assert 1–8）は緑で、本 issue は `range`（PRED）に固有。
