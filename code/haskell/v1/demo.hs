-- λ-1 translator — Haskell prelude
module Main where

import System.Exit (exitFailure)
import Data.List (intercalate)

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

-- ============ JSON 層（型付き値。ADR-0005） ============
jK :: D
jK = Fun (\a -> Fun (\_ -> a))
jKI :: D
jKI = Fun (\_ -> Fun (\b -> b))
mkpair :: D -> D -> D
mkpair a b = Fun (\s -> app (app s a) b)
fstp :: D -> D
fstp p = app p jK
sndp :: D -> D
sndp p = app p jKI
churchToInt :: D -> Int
churchToInt c = case app (app c incr) (Num 0) of Num k -> k; _ -> error "churchToInt"
cTrue :: D
cTrue = Fun (\t -> Fun (\_ -> t))
cFalse :: D
cFalse = Fun (\_ -> Fun (\f -> f))
boolToHost :: D -> Bool
boolToHost c = case app (app c (Str "T")) (Str "F") of Str "T" -> True; _ -> False
nilH :: D
nilH = Fun (\n -> Fun (\_ -> n))
consH :: D -> D -> D
consH h t = Fun (\_ -> Fun (\c -> app (app c h) t))
isNil :: D -> Bool
isNil lst = boolToHost (app (app lst cTrue) (Fun (\_ -> Fun (\_ -> cFalse))))
headL :: D -> D
headL lst = app (app lst (Str "")) (Fun (\h -> Fun (\_ -> h)))
tailL :: D -> D
tailL lst = app (app lst (Str "")) (Fun (\_ -> Fun (\t -> t)))
walkL :: D -> [D]
walkL lst = if isNil lst then [] else headL lst : walkL (tailL lst)

jInt :: Int -> D
jInt n = mkpair (encodeInt 1) (encodeInt n)
jBool :: D -> D
jBool b = mkpair (encodeInt 2) b
jStr :: String -> D
jStr s = mkpair (encodeInt 3) (foldr (\ch lst -> consH (encodeInt (fromEnum ch)) lst) nilH s)
jArr :: D -> D
jArr lst = mkpair (encodeInt 4) lst
jObj :: D -> D
jObj lst = mkpair (encodeInt 5) lst
jNull :: D
jNull = mkpair (encodeInt 6) (Fun (\x -> x))

jsonEscape :: String -> String
jsonEscape s = "\"" ++ concatMap esc s ++ "\""
  where esc '"' = "\\\""
        esc '\\' = "\\\\"
        esc c = [c]

decodeJson :: D -> String
decodeJson v =
  let tag = churchToInt (fstp v)
      payload = sndp v
  in case tag of
       1 -> show (churchToInt payload)
       2 -> if boolToHost payload then "true" else "false"
       3 -> jsonEscape (map (toEnum . churchToInt) (walkL payload))
       4 -> "[" ++ intercalate "," (map decodeJson (walkL payload)) ++ "]"
       5 -> "{" ++ intercalate "," (map (\pr -> decodeJson (fstp pr) ++ ":" ++ decodeJson (sndp pr)) (walkL payload)) ++ "}"
       6 -> "null"
       _ -> "?"

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
