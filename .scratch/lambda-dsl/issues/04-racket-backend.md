# 04: Racket バックエンド

Status: ready-for-agent

依存: [03](03-python-backend-runner.md)

## 内容

Lisp-1・sexpr 系の代表として Racket バックエンドを実装する。

## 仕様

- `λx.M` → `(lambda (_x) M)`、適用 → `(f x)`、定義 → `(define _name expr)`、万能型不要
- プレリュード `translator/preludes/racket.rkt`: `encodeInt`/`decodeInt`/`decodeBool`/`myassert`。`decode*` は文字列を返す（`number->string` 等）
- プログラム外枠: `#lang racket` ＋ 定義列 ＋ assert 列

## 受け入れ条件

- v1 コーパスを Racket 生成・実行し全 assert 緑（`nix develop` 下の `racket`）
