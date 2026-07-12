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
val v_I = (Fun (fn v_x => v_x))
val v_K = (Fun (fn v_x => (Fun (fn v_y => v_x))))
val v_S = (Fun (fn v_x => (Fun (fn v_y => (Fun (fn v_z => (app (app v_x v_z) (app v_y v_z))))))))
val v_zero = (Fun (fn v_f => (Fun (fn v_x => v_x))))
val v_succ = (Fun (fn v_n => (Fun (fn v_f => (Fun (fn v_x => (app v_f (app (app v_n v_f) v_x))))))))
val v_add = (Fun (fn v_m => (Fun (fn v_n => (Fun (fn v_f => (Fun (fn v_x => (app (app v_m v_f) (app (app v_n v_f) v_x))))))))))
val v_true = (Fun (fn v_t => (Fun (fn v_f => v_t))))
val v_false = (Fun (fn v_t => (Fun (fn v_f => v_f))))
val v_if = (Fun (fn v_b => (Fun (fn v_t => (Fun (fn v_e => (app (app v_b v_t) v_e)))))))
val v_and = (Fun (fn v_p => (Fun (fn v_q => (app (app v_p v_q) v_p)))))
val v_not = (Fun (fn v_b => (app (app v_b v_false) v_true)))

(* --- assertions --- *)
val _ = check "assert 1" ("1") ((decodeInt (app (app (app v_S v_K) v_K) (encodeInt 1))))
val _ = check "assert 2" ("0") ((decodeInt v_zero))
val _ = check "assert 3" ("3") ((decodeInt (app v_succ (encodeInt 2))))
val _ = check "assert 4" ("2") ((decodeInt (app (app v_add (encodeInt 1)) (encodeInt 1))))
val _ = check "assert 5" ("true") ((decodeBool (app (app v_and v_true) v_true)))
val _ = check "assert 6" ("false") ((decodeBool (app (app v_and v_true) v_false)))
val _ = check "assert 7" ("false") ((decodeBool (app (app (app v_if v_false) v_true) v_false)))
val _ = check "assert 8" ("true") ((decodeBool (app v_not v_false)))

val _ = finish ()
