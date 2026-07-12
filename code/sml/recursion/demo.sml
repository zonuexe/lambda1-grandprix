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
val v_Z = (Fun (fn v_f => (app (Fun (fn v_x => (app v_f (Fun (fn v_v => (app (app v_x v_x) v_v)))))) (Fun (fn v_x => (app v_f (Fun (fn v_v => (app (app v_x v_x) v_v)))))))))
val v_one = (Fun (fn v_f => (Fun (fn v_x => (app v_f v_x)))))
val v_mult = (Fun (fn v_m => (Fun (fn v_n => (Fun (fn v_f => (app v_m (app v_n v_f))))))))
val v_pred = (Fun (fn v_n => (Fun (fn v_f => (Fun (fn v_x => (app (app (app v_n (Fun (fn v_g => (Fun (fn v_h => (app v_h (app v_g v_f))))))) (Fun (fn v_u => v_x))) (Fun (fn v_u => v_u)))))))))
val v_true = (Fun (fn v_t => (Fun (fn v_f => v_t))))
val v_false = (Fun (fn v_t => (Fun (fn v_f => v_f))))
val v_isZero = (Fun (fn v_n => (app (app v_n (Fun (fn v_x => v_false))) v_true)))
val v_fstep = (Fun (fn v_rec => (Fun (fn v_n => (app (app (app (app v_isZero v_n) (Fun (fn v_u => v_one))) (Fun (fn v_u => (app (app v_mult v_n) (app v_rec (app v_pred v_n)))))) v_n)))))
val v_fact = (app v_Z v_fstep)

(* --- assertions --- *)
val _ = check "assert 1" ("1") ((decodeInt (app v_fact (encodeInt 0))))
val _ = check "assert 2" ("1") ((decodeInt (app v_fact (encodeInt 1))))
val _ = check "assert 3" ("2") ((decodeInt (app v_fact (encodeInt 2))))
val _ = check "assert 4" ("6") ((decodeInt (app v_fact (encodeInt 3))))
val _ = check "assert 5" ("120") ((decodeInt (app v_fact (encodeInt 5))))

val _ = finish ()
