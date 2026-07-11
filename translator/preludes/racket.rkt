#lang racket
;; λ-1 translator — Racket prelude
(provide (all-defined-out))
(require json)

(define _failures 0)

(define (encodeInt n)          ; host int -> チャーチ数
  (lambda (f)
    (lambda (x)
      (let loop ((k n) (acc x))
        (if (= k 0) acc (loop (- k 1) (f acc)))))))

(define (decodeInt t)          ; チャーチ数 -> 文字列
  (number->string ((t (lambda (k) (+ k 1))) 0)))

(define (decodeBool t)         ; チャーチ真偽値 -> "true"/"false"
  ((t "true") "false"))

(define (_check a b label)
  (if (equal? a b)
      (printf "ok   ~a\n" label)
      (begin (set! _failures (+ _failures 1))
             (printf "FAIL ~a: ~s != ~s\n" label a b))))

(define (_finish)
  (when (> _failures 0)
    (printf "~a failure(s)\n" _failures)
    (exit 1))
  (printf "all green\n"))


;; ============ JSON 層（型付き値。ADR-0005） ============
(define (jK a) (lambda (b) a))
(define (jKI a) (lambda (b) b))
(define (mkpair a b) (lambda (s) ((s a) b)))
(define (fstp p) (p jK))
(define (sndp p) (p jKI))
(define (churchToInt c) ((c (lambda (k) (+ k 1))) 0))
(define (jcTrue t) (lambda (f) t))
(define (jcFalse t) (lambda (f) f))
(define (boolToHost c) ((c #t) #f))
(define nilH (lambda (n) (lambda (c) n)))
(define (consH h t) (lambda (n) (lambda (c) ((c h) t))))
(define (isNil lst) (boolToHost ((lst jcTrue) (lambda (h) (lambda (t) jcFalse)))))
(define (headL lst) ((lst #f) (lambda (h) (lambda (t) h))))
(define (tailL lst) ((lst #f) (lambda (h) (lambda (t) t))))
(define (walk lst)
  (let loop ((lst lst) (acc '()))
    (if (isNil lst) (reverse acc) (loop (tailL lst) (cons (headL lst) acc)))))

(define (jInt n) (mkpair (encodeInt 1) (encodeInt n)))
(define (jBool b) (mkpair (encodeInt 2) b))
(define (jStr s)
  (mkpair (encodeInt 3)
          (foldr (lambda (byte lst) (consH (encodeInt byte) lst))
                 nilH (bytes->list (string->bytes/utf-8 s)))))
(define (jArr lst) (mkpair (encodeInt 4) lst))
(define (jObj lst) (mkpair (encodeInt 5) lst))
(define (jNull) (mkpair (encodeInt 6) (lambda (x) x)))

(define (decodeJson v)
  (define tag (churchToInt (fstp v)))
  (define payload (sndp v))
  (cond
    ((= tag 1) (number->string (churchToInt payload)))
    ((= tag 2) (if (boolToHost payload) "true" "false"))
    ((= tag 3) (jsexpr->string
                (bytes->string/utf-8 (list->bytes (map churchToInt (walk payload))))))
    ((= tag 4) (string-append "[" (string-join (map decodeJson (walk payload)) ",") "]"))
    ((= tag 5) (string-append "{"
                (string-join (map (lambda (pr)
                                    (string-append (decodeJson (fstp pr)) ":" (decodeJson (sndp pr))))
                                  (walk payload)) ",") "}"))
    ((= tag 6) "null")
    (else "?")))
