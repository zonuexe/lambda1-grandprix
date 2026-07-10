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
