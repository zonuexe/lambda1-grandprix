-- λ-1 translator — Haskell prelude
module Main where

import System.Exit (exitFailure)

-- 万能型（タグ付き union）。ラムダ世界の値は常に Fun。
-- 地の構築子 Num/Str は encode/decode の境界でのみ現れる。
data D = Fun (D -> D) | Num Int | Str String

app :: D -> D -> D
app (Fun f) x = f x
app _ _ = error "applied a non-function"

encodeInt :: Int -> D                 -- host int -> チャーチ数（Fun）
encodeInt n = Fun (\f -> Fun (\x -> go n f x))
  where
    go 0 _ x = x
    go k f x = go (k - 1) f (app f x)

incr :: D
incr = Fun step
  where
    step (Num k) = Num (k + 1)
    step _ = error "incr: not a number"

decodeInt :: D -> String              -- チャーチ数 -> 文字列（Num を注入して数える）
decodeInt t = case app (app t incr) (Num 0) of
  Num k -> show k
  _ -> error "decodeInt: not a number"

decodeBool :: D -> String             -- チャーチ真偽値 -> "true"/"false"（Str を注入）
decodeBool t = case app (app t (Str "true")) (Str "false") of
  Str s -> s
  _ -> error "decodeBool: not a bool"

check :: String -> String -> String -> IO Bool
check label a b =
  if a == b
    then putStrLn ("ok   " ++ label) >> return True
    else putStrLn ("FAIL " ++ label ++ ": " ++ show a ++ " /= " ++ show b) >> return False

-- --- definitions ---
_I = (Fun (\_x -> _x))
_K = (Fun (\_x -> (Fun (\_y -> _x))))
_S = (Fun (\_x -> (Fun (\_y -> (Fun (\_z -> (app (app _x _z) (app _y _z))))))))
_zero = (Fun (\_f -> (Fun (\_x -> _x))))
_succ = (Fun (\_n -> (Fun (\_f -> (Fun (\_x -> (app _f (app (app _n _f) _x))))))))
_add = (Fun (\_m -> (Fun (\_n -> (Fun (\_f -> (Fun (\_x -> (app (app _m _f) (app (app _n _f) _x))))))))))
_true = (Fun (\_t -> (Fun (\_f -> _t))))
_false = (Fun (\_t -> (Fun (\_f -> _f))))
_if = (Fun (\_b -> (Fun (\_t -> (Fun (\_e -> (app (app _b _t) _e)))))))
_and = (Fun (\_p -> (Fun (\_q -> (app (app _p _q) _p)))))
_not = (Fun (\_b -> (app (app _b _false) _true)))

-- --- main ---
main :: IO ()
main = do
  results <- sequence
    [ check "assert 1" ("1") ((decodeInt (app (app (app _S _K) _K) (encodeInt 1))))
    , check "assert 2" ("0") ((decodeInt _zero))
    , check "assert 3" ("3") ((decodeInt (app _succ (encodeInt 2))))
    , check "assert 4" ("2") ((decodeInt (app (app _add (encodeInt 1)) (encodeInt 1))))
    , check "assert 5" ("true") ((decodeBool (app (app _and _true) _true)))
    , check "assert 6" ("false") ((decodeBool (app (app _and _true) _false)))
    , check "assert 7" ("false") ((decodeBool (app (app (app _if _false) _true) _false)))
    , check "assert 8" ("true") ((decodeBool (app _not _false)))
    ]
  let fails = length (filter not results)
  if fails > 0 then putStrLn (show fails ++ " failure(s)") >> exitFailure else putStrLn "all green"
