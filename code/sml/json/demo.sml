(* λ-1 translator — Standard ML prelude (MLton) *)

datatype D = Fun of D -> D | Num of int | Str of string

fun app (Fun f) x = f x
  | app _ _ = raise Fail "applied a non-function"

fun encodeInt n =                 (* host int -> チャーチ数 *)
  Fun (fn f => Fun (fn x =>
    let
      fun go 0 acc = acc
        | go k acc = go (k - 1) (app f acc)
    in go n x end))

fun decodeInt t =                 (* Num を注入して数える *)
  let
    val incr = Fun (fn v => case v of Num k => Num (k + 1) | _ => raise Fail "incr")
  in
    case app (app t incr) (Num 0) of Num k => Int.toString k | _ => raise Fail "decodeInt"
  end

fun decodeBool t =                (* Str を注入 *)
  case app (app t (Str "true")) (Str "false") of Str s => s | _ => raise Fail "decodeBool"

val failures = ref 0

fun check label a b =
  if a = b then print ("ok   " ^ label ^ "\n")
  else (failures := !failures + 1;
        print ("FAIL " ^ label ^ ": " ^ a ^ " != " ^ b ^ "\n"))

fun finish () =
  if !failures > 0 then
    (print (Int.toString (!failures) ^ " failure(s)\n"); OS.Process.exit OS.Process.failure)
  else print "all green\n"

(* ============ JSON 層（型付き値。ADR-0005） ============ *)
val jKsel  = Fun (fn a => Fun (fn _ => a))
val jKIsel = Fun (fn _ => Fun (fn b => b))
fun mkpair a b = Fun (fn s => app (app s a) b)
fun fstp p = app p jKsel
fun sndp p = app p jKIsel

val churchIncr = Fun (fn v => case v of Num k => Num (k + 1) | _ => raise Fail "churchIncr")
fun churchToInt c = case app (app c churchIncr) (Num 0) of Num k => k | _ => raise Fail "churchToInt"

val cTrue  = Fun (fn t => Fun (fn _ => t))
val cFalse = Fun (fn _ => Fun (fn f => f))
fun boolToHost c = case app (app c (Str "T")) (Str "F") of Str "T" => true | _ => false

val nilH = Fun (fn n => Fun (fn _ => n))
fun consH h t = Fun (fn _ => Fun (fn c => app (app c h) t))
fun isNil lst = boolToHost (app (app lst cTrue) (Fun (fn _ => Fun (fn _ => cFalse))))
fun headL lst = app (app lst (Str "")) (Fun (fn h => Fun (fn _ => h)))
fun tailL lst = app (app lst (Str "")) (Fun (fn _ => Fun (fn t => t)))
fun walkL lst = if isNil lst then [] else headL lst :: walkL (tailL lst)

fun jInt n = mkpair (encodeInt 1) (encodeInt n)
fun jBool b = mkpair (encodeInt 2) b
fun jStr s =
  mkpair (encodeInt 3)
    (List.foldr (fn (ch, lst) => consH (encodeInt (Char.ord ch)) lst) nilH (String.explode s))
fun jArr lst = mkpair (encodeInt 4) lst
fun jObj lst = mkpair (encodeInt 5) lst
val jNull = mkpair (encodeInt 6) (Fun (fn x => x))

fun jsonEscape s =
  let
    fun esc c =
      if c = #"\"" then "\\\""
      else if c = #"\\" then "\\\\"
      else String.str c
  in
    "\"" ^ String.concat (map esc (String.explode s)) ^ "\""
  end

fun decodeJson v =
  let
    val tag = churchToInt (fstp v)
    val payload = sndp v
  in
    case tag of
        1 => Int.toString (churchToInt payload)
      | 2 => if boolToHost payload then "true" else "false"
      | 3 => jsonEscape (String.implode (map (fn c => Char.chr (churchToInt c)) (walkL payload)))
      | 4 => "[" ^ String.concatWith "," (map decodeJson (walkL payload)) ^ "]"
      | 5 => "{" ^ String.concatWith ","
               (map (fn pr => decodeJson (fstp pr) ^ ":" ^ decodeJson (sndp pr)) (walkL payload)) ^ "}"
      | 6 => "null"
      | _ => "?"
  end

(* --- definitions --- *)
val v_pair = (Fun (fn v_a => (Fun (fn v_b => (Fun (fn v_s => (app (app v_s v_a) v_b)))))))
val v_nil = (Fun (fn v_n => (Fun (fn v_c => v_n))))
val v_cons = (Fun (fn v_h => (Fun (fn v_t => (Fun (fn v_n => (Fun (fn v_c => (app (app v_c v_h) v_t)))))))))
val v_true = (Fun (fn v_t => (Fun (fn v_f => v_t))))
val v_false = (Fun (fn v_t => (Fun (fn v_f => v_f))))
val v_snd = (Fun (fn v_p => (app v_p v_false)))
val v_one = (Fun (fn v_f => (Fun (fn v_x => (app v_f v_x)))))
val v_pred = (Fun (fn v_n => (Fun (fn v_f => (Fun (fn v_x => (app (app (app v_n (Fun (fn v_g => (Fun (fn v_h => (app v_h (app v_g v_f))))))) (Fun (fn v_u => v_x))) (Fun (fn v_u => v_u)))))))))
val v_tint = (Fun (fn v_k => (app (app v_pair v_one) v_k)))
val v_step = (Fun (fn v_p => (app v_p (Fun (fn v_k => (Fun (fn v_l => (app (app v_pair (app v_pred v_k)) (app (app v_cons (app v_tint v_k)) v_l)))))))))
val v_range = (Fun (fn v_n => (app v_snd (app (app v_n v_step) (app (app v_pair v_n) v_nil)))))

(* --- assertions --- *)
val _ = check "assert 1" ("1") ((decodeJson (jInt 1)))
val _ = check "assert 2" ("true") ((decodeJson (jBool v_true)))
val _ = check "assert 3" ("false") ((decodeJson (jBool v_false)))
val _ = check "assert 4" ("\"hi\"") ((decodeJson (jStr "hi")))
val _ = check "assert 5" ("null") ((decodeJson jNull))
val _ = check "assert 6" ("[1,true]") ((decodeJson (jArr (app (app v_cons (jInt 1)) (app (app v_cons (jBool v_true)) v_nil)))))
val _ = check "assert 7" ("{\"k\":1}") ((decodeJson (jObj (app (app v_cons (app (app v_pair (jStr "k")) (jInt 1))) v_nil))))
val _ = check "assert 8" ("[1,[2,3]]") ((decodeJson (jArr (app (app v_cons (jInt 1)) (app (app v_cons (jArr (app (app v_cons (jInt 2)) (app (app v_cons (jInt 3)) v_nil)))) v_nil)))))
val _ = check "assert 9" ("[]") ((decodeJson (jArr (app v_range (encodeInt 0)))))
val _ = check "assert 10" ("[1]") ((decodeJson (jArr (app v_range (encodeInt 1)))))
val _ = check "assert 11" ("[1,2,3]") ((decodeJson (jArr (app v_range (encodeInt 3)))))

val _ = finish ()
