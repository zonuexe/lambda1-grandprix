# 型付き値（タグ付き）と JSON 相当構造の表現

純ラムダの値に**型タグ**を付け、汎用 `decode` が自己記述的に JSON 相当（null / bool / number / string / array / object）へ落とせるようにする。既存の raw 層（[[0004-lambda-dsl-and-translator]] の `encodeInt`/`decodeInt` 等、生のチャーチ数・真偽値）はそのまま残し、その上に**JSON 層**として**併存**で追加する。`float` は id を予約するのみで**実装・内部表現とも延期**。

## 値の表現

**型付き値 = `pair tag payload`**（チャーチペア `pair = λa.λb.λs. s a b`）。tag はチャーチ数の id。

| id | 型 | payload | JSON |
|--|--|--|--|
| 1 | int | チャーチ数 | number |
| 2 | bool | チャーチ真偽値（`λt.λf.t` / `λt.λf.f`） | true / false |
| 3 | string | **バイト列**（チャーチ数のリスト、各 0–255） | string |
| 4 | array | 型付き値のリスト（再帰） | array |
| 5 | object | `(key, value)` ペアのリスト。**key は id=3 の型付き string**、value は型付き値（再帰） | object |
| 6 | null | なし（payload は任意、例 `I`） | null |
| 7 | float | **予約・未実装（内部表現も未定）** | number |

## リストのエンコーディング: Scott（採用）

array / string(バイト列) / object の土台となるリストは **Scott エンコーディング**を採用:
`nil = λn.λc. n`、`cons h t = λn.λc. c h t`。

### Considered Options（pros/cons）

- **Scott（採用）**
  - Pros: `list onNil onCons` で即座に場合分けでき、`onCons` が head・tail を直接受け取る。head/tail が O(1)。ホスト側の**要素ごとの再帰 decode に最適**。遅延評価と相性良（全体を強制しない）。
  - Cons: それ自体は fold ではないので、集約（length / sum / map）は自前再帰＝純ラムダでは Y（strict 下では Z）が要る。
- **Church（fold）エンコーディング**（`nil = λc.λn. n`、`cons h t = λc.λn. c h (t c n)`）
  - Pros: リスト自身が右 fold。`list f z` で集約が Y なしに書ける。計算向きで最も正準。
  - Cons: **分解（head/tail）が苦手**。tail の取り出しは O(n)（paramorphism 的再構築）。要素ごとの decode に不向き。strict 言語では fold 全体を強制。
- **pair + nil センチネル**（`cons = λf. f h t` ＋ nil 用識別値）
  - Pros: 直感的（連結ペア）。head/tail は fst/snd で O(1)。
  - Cons: nil と cons の**識別が自前規約頼み**で脆い（有効データと衝突しうる）。実質「Scott の識別を雑にやったもの」。

→ 本用途は**構造の decode**（ホストが走査して JSON を構築）なので、O(1) 分解と場合分けが素直な **Scott が最適**。集約が要る場面は限定的で、その時だけ Y/Z を使えばよい。

## 汎用 decode（ホスト側・再帰）

`decode` は tag を読み（チャーチ数 → ホスト int）、分岐して payload を再帰的に JSON 文字列へ落とす:
- 1 int → 数値 / 2 bool → true/false / 3 string → バイト列を文字列化 / 4 array → 要素を再帰 decode して `[...]` / 5 object → (key,value) を再帰して `{...}` / 6 null → `null`。
- 戻り値は JSON 文字列（[[0004-lambda-dsl-and-translator]] の「decode は文字列を返す」方針を継続。assert の比較対象）。

型付き言語の[[CONTEXT|万能型]]は現状の `Fun | Num | Str` のままで実装可能:
- tag / int / byte はチャーチ数 → `Num` 注入で読む
- bool は真偽マーカー（host の 2 値）を注入して読む
- リストは Scott の selector（`isNil = list T (λh.λt. F)`、`head = list dummy (λh.λt. h)`、`tail = list dummy (λh.λt. t)`）で head/tail を `Fun`（＝D）として取り出し、ホストが再帰

## Consequences / 保留

- **float は id=7 予約のみ**。実装も内部表現（IEEE754 をどうチャーチ/バイトで保持するか等）も延期。
- 文字列は**バイト忠実**。UTF-8 解釈は JSON 出力時にホストが行う想定で、マルチバイトの厳密な扱いは詰める余地。
- encode 側は型ごとの構築子（`int[]` / `bool[]` / `str[]` / `list[]` / `obj[]` / `null`）＋汎用 `decode`。raw 層（`encodeInt`/`decodeInt`）と併存。構築子の**正確な名前は実装時に確定**（raw 層と衝突しない命名。JSON リテラル糖衣は将来）。
- DSL 内で length 等の集約を書くなら Y/Z が必要（純ラムダの反復＝別デモの題材になる）。
