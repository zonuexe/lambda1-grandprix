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
