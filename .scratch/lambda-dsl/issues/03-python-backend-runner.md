# 03: Python バックエンド ＋ 実行ランナー

Status: ready-for-agent

依存: [02](02-codegen-core.md)

## 内容

最初の（動的・基準）バックエンドとして Python を実装し、生成→実行→assert 判定を回すランナーを作る。

## 仕様

- Python バックエンド: `λx.M` → `lambda _x: M`、適用 → `f(x)`、定義 → `_name = expr`、万能型不要（素のクロージャ）
- プレリュード `translator/preludes/python.py`: `encodeInt(n)`（host int → チャーチ数クロージャ）、`decodeInt(t)`（チャーチ数 → 文字列。`t(lambda n: n+1)(0)` を str 化）、`decodeBool(t)`（`t('true')('false')`）、`myassert(b)`（native `assert` を使うなら不要）
- ランナー: `translator run <dsl>` で全対象言語のソースを出力ディレクトリに生成し、各言語処理系で実行して assert 結果（緑/赤）を集約表示

## 受け入れ条件

- v1 コーパス断片（`assert '1' == decodeInt[ S K K (encodeInt[1]) ]` 等）を Python 生成・実行し緑
- `nix develop` 下で `python3` を用いて自動実行される
