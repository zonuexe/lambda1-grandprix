# 08: v1 コーパス ＋ 全5言語で assert 緑

Status: ready-for-agent

依存: [03](03-python-backend-runner.md), [04](04-racket-backend.md), [05](05-haskell-backend.md), [06](06-java-backend.md), [07](07-rust-backend.md)

## 内容

v1 の DSL コーパスを整備し、5言語すべてで assert が緑になることを CI 的に確認する。

## コーパス（`translator/corpus/v1.lam` 目安）

```
I = λx.x
K = λx.λy.x
S = λx.λy.λz.x z (y z)

zero = λf.λx.x
succ = λn.λf.λx.f (n f x)
add  = λm.λn.λf.λx.m f (n f x)

true  = λt.λf.t
false = λt.λf.f
if    = λb.λt.λe.b t e
and   = λp.λq.p q p
not   = λb.b false true
---
assert '1' == decodeInt[ S K K (encodeInt[1]) ]
assert '0' == decodeInt[ zero ]
assert '3' == decodeInt[ succ (encodeInt[2]) ]
assert '2' == decodeInt[ add (encodeInt[1]) (encodeInt[1]) ]
assert 'true'  == decodeBool[ and true true ]
assert 'false' == decodeBool[ and true false ]
assert 'false' == decodeBool[ if false true false ]
assert 'true'  == decodeBool[ not false ]
```

## 受け入れ条件

- `translator run translator/corpus/v1.lam` が Python/Racket/Haskell/Java/Rust を生成・実行し、全 assert 緑
- 1コマンドで5言語まとめて回せる（`nix develop` 前提）
