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
