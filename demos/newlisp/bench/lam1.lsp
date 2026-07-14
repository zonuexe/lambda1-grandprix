; λ-1 translator — niiLISP / newLISP prelude（クロージャ変換 / defunctionalized）
;
; DSL の λ はトップレベル関数 Lk(_env x) ＋捕捉環境データ (list Lk cap...) へ変換される
; （src/codegen/newlisp.rs、issues/14 approach C）。適用は一律 APPLY。ネストしたネイティブ
; lambda を作らないので、動的スコープの niiLISP/newLISP でも決定的に正しく動く。

; クロージャ適用: ネイティブ関数（プレリュードが注入する inc 等）とデータクロージャの両対応。
(define (APPLY clo x)
  (if (lambda? clo)
      (clo x)
      ((first clo) (rest clo) x)))

; ---- raw 層 ----
; チャーチ数 N = λf.λx. f^N x を defunctionalized で構築。
(define (L_ch_x _env x)                                ; _env=(N f)
  (let ((n (first _env)) (f (first (rest _env))) (acc x))
    (dotimes (i n) (set 'acc (APPLY f acc)))
    acc))
(define (L_ch_f _env f) (list L_ch_x (first _env) f))   ; _env=(N) -> λx. f^N x
(define (encodeInt N) (list L_ch_f N))                ; host int -> チャーチ数

(define (decodeInt v)                                 ; チャーチ数 -> 文字列
  (string (APPLY (APPLY v (fn (k) (+ k 1))) 0)))
(define (decodeBool v)                                ; チャーチ真偽値 -> "true"/"false"
  (APPLY (APPLY v "true") "false"))

(set 'failures 0)
(define (_check a b label)
  (if (= a b)
      (println "ok   " label)
      (begin (inc failures) (println "FAIL " label ": " a " != " b))))
(define (_finish)
  (if (> failures 0)
      (begin (println failures " failure(s)") (exit 1))
      (begin (println "all green") (exit))))

; ---- JSON 層（型付き値。ADR-0005）defunctionalized ----
; セレクタ: K=λa.λb.a, KI=λa.λb.b, ID=λx.x
(define (L_k2 _env b) (first _env))                 ; _env=(a) -> a
(define (L_k1 _env a) (list L_k2 a))               ; λa.λb.a
(define K_SEL (list L_k1))
(define (L_id _env x) x)                            ; λx.x
(define (L_ki1 _env a) (list L_id))                ; λa.λb.b = λa.ID
(define KI_SEL (list L_ki1))

; pair a b = λs. s a b
(define (L_pair _env s)                            ; _env=(a b)
  (APPLY (APPLY s (first _env)) (first (rest _env))))
(define (mkpair a b) (list L_pair a b))
(define (fstp p) (APPLY p K_SEL))
(define (sndp p) (APPLY p KI_SEL))

(define (churchToInt c) (APPLY (APPLY c (fn (k) (+ k 1))) 0))
(define (boolToHost c) (APPLY (APPLY c true) nil))    ; true / nil

; Scott リスト: nil=λn.λc.n=K, cons h t=λn.λc. c h t
(define NIL_H K_SEL)
(define (L_cons2 _env c)                           ; _env=(h t) -> c h t
  (APPLY (APPLY c (first _env)) (first (rest _env))))
(define (L_cons1 _env n) (list L_cons2 (first _env) (first (rest _env))))  ; _env=(h t), ignore n
(define (consH h t) (list L_cons1 h t))

; isNil: nil-case=cTrue(K), cons-case=λh.λt.cFalse(KI)
(define (L_fc2 _env t) KI_SEL)
(define (L_fc1 _env h) (list L_fc2))               ; λh.λt.KI
(define (isNil lst) (boolToHost (APPLY (APPLY lst K_SEL) (list L_fc1))))
(define (headL lst) (APPLY (APPLY lst "") K_SEL))   ; cons-case λh.λt.h = K
(define (tailL lst) (APPLY (APPLY lst "") KI_SEL))  ; cons-case λh.λt.t = KI
(define (walk lst)
  (let ((out '()))
    (while (not (isNil lst))
      (push (headL lst) out -1)
      (set 'lst (tailL lst)))
    out))

(define (jInt n)  (mkpair (encodeInt 1) (encodeInt n)))
(define (jBool b) (mkpair (encodeInt 2) b))
(define (jStr s)
  (let ((lst NIL_H))
    (dolist (code (reverse (map char (explode s))))
      (set 'lst (consH (encodeInt code) lst)))
    (mkpair (encodeInt 3) lst)))
(define (jArr lst) (mkpair (encodeInt 4) lst))
(define (jObj lst) (mkpair (encodeInt 5) lst))
(define (jNull)    (mkpair (encodeInt 6) (list L_id)))

(define (jsonEscape s)
  (let ((out "\""))
    (dolist (ch (explode s))
      (cond ((= ch "\"") (extend out "\\\""))
            ((= ch "\\") (extend out "\\\\"))
            (true        (extend out ch))))
    (extend out "\"")))
(define (decodeJson v)
  (let ((tag (churchToInt (fstp v))) (payload (sndp v)))
    (cond
      ((= tag 1) (string (churchToInt payload)))
      ((= tag 2) (if (boolToHost payload) "true" "false"))
      ((= tag 3) (jsonEscape (join (map (fn (b) (char (churchToInt b))) (walk payload)))))
      ((= tag 4) (append "[" (join (map decodeJson (walk payload)) ",") "]"))
      ((= tag 5) (append "{"
                   (join (map (fn (pr) (append (decodeJson (fstp pr)) ":" (decodeJson (sndp pr))))
                              (walk payload)) ",") "}"))
      ((= tag 6) "null")
      (true "?"))))
