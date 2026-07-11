; λ-1 translator — newLISP / niiLISP prelude
;
; niiLISP は cons 無し・ダイナミックスコープの独特な LISP。ラムダ計算はレキシカル
; クロージャを要するので、kosh04 の手法を応用する:
;   define-macro (fexpr) の LAMBDA が、生成時に expand で「大文字の自由変数」を
;   その値に焼き込み、ダイナミックスコープ下でも疑似レキシカルクロージャを作る。
;   → 変数は大文字にマングリングする（expand は大文字シンボルを展開する）。
; https://gist.github.com/kosh04/262332
(define-macro (LAMBDA) (append (lambda) (expand (args))))

(define (encodeInt N)            ; host int -> チャーチ数（N は大文字＝焼き込み対象）
  (LAMBDA (F) (LAMBDA (X)
    (let (acc X)
      (dotimes (i N) (set 'acc (F acc)))
      acc))))

(define (decodeInt v)            ; チャーチ数 -> 文字列
  (string ((v (fn (k) (+ k 1))) 0)))

(define (decodeBool v)           ; チャーチ真偽値 -> "true"/"false"
  ((v "true") "false"))

(set 'failures 0)

(define (_check a b label)
  (if (= a b)
      (println "ok   " label)
      (begin
        (inc failures)
        (println "FAIL " label ": " a " != " b))))

(define (_finish)
  (if (> failures 0)
      (begin (println failures " failure(s)") (exit 1))
      (begin (println "all green") (exit))))


; ============ JSON 層（型付き値。ADR-0005） ============
; ダイナミックスコープ＋expand（大文字のみ焼き込み）の落とし穴に注意:
;   閉包に「捕捉」される自由変数だけを大文字にし、しかも全域で一意名にする
;   （render の V0,V1… と同じ理由。同名の大文字が別フレームで束縛されると
;    expand が誤った値を lambda 本体・仮引数位置に焼き込んで壊れる）。
;   捕捉されない変数は小文字にする（小文字は expand 対象外なので安全）。
;   捕捉大文字名は encodeInt の N/F/X や DSL 大域名(PAIR,NIL,…)と衝突させない。
; 既知の制約: json.lam の assert 10/11（range 経由の [1] / [1,2,3]）は緑にできない。
;   これは JSON 層ではなく render_scoped（newlisp.rs）側の疑似クロージャの限界:
;   expand が大文字大域コンビネータ(PAIR/CONS/TINT)を閉包へインライン展開し、
;   その内部で再利用される V0,V1… が range の入れ子簡約で衝突して壊れる。
;   assert 1-9 は緑。JSON プレリュード単体では回避不能（render 変更が要る）。
(define (jK JKA)     (LAMBDA (b) JKA))
(define (jKI a)      (LAMBDA (b) b))
(define (mkpair MPA MPB) (LAMBDA (s) ((s MPA) MPB)))
(define (fstp p)     (p jK))
(define (sndp p)     (p jKI))
(define (churchToInt c) ((c (fn (k) (+ k 1))) 0))
(define (jcTrue JCT) (LAMBDA (f) JCT))
(define (jcFalse t)  (LAMBDA (f) f))
(define (boolToHost c) ((c true) nil))
(define nilH         (LAMBDA (NLN) (LAMBDA (c) NLN)))
(define (consH CHH CHT) (LAMBDA (n) (LAMBDA (c) ((c CHH) CHT))))
(define (isNil lst)
  (boolToHost ((lst jcTrue) (LAMBDA (h) (LAMBDA (tl) jcFalse)))))
(define (headL lst) ((lst nil) (LAMBDA (HLH) (LAMBDA (tl) HLH))))
(define (tailL lst) ((lst nil) (LAMBDA (h) (LAMBDA (tl) tl))))
(define (walk lst)
  (let (acc '())
    (while (not (isNil lst))
      (push (headL lst) acc -1)
      (set 'lst (tailL lst)))
    acc))

(define (jInt n)  (mkpair (encodeInt 1) (encodeInt n)))
(define (jBool b) (mkpair (encodeInt 2) b))
(define (jStr s)
  (mkpair (encodeInt 3)
    (let (lst nilH)
      (dolist (ch (reverse (map char (explode s))))
        (set 'lst (consH (encodeInt ch) lst)))
      lst)))
(define (jArr l) (mkpair (encodeInt 4) l))
(define (jObj l) (mkpair (encodeInt 5) l))
(define (jNull)  (mkpair (encodeInt 6) (LAMBDA (x) x)))

(define (jsonEscape s)
  (let (out "\"")
    (dolist (ch (explode s))
      (set 'out (append out
        (cond ((= ch "\"") "\\\"")
              ((= ch "\\") "\\\\")
              (true ch)))))
    (append out "\"")))

(define (decodeJson v)
  (let (tag (churchToInt (fstp v)) payload (sndp v))
    (cond
      ((= tag 1) (string (churchToInt payload)))
      ((= tag 2) (if (boolToHost payload) "true" "false"))
      ((= tag 3) (jsonEscape (join (map char (map churchToInt (walk payload))))))
      ((= tag 4) (append "[" (join (map decodeJson (walk payload)) ",") "]"))
      ((= tag 5) (append "{"
                   (join (map (fn (pr)
                                (append (decodeJson (fstp pr)) ":" (decodeJson (sndp pr))))
                              (walk payload)) ",")
                   "}"))
      ((= tag 6) "null")
      (true "?"))))
