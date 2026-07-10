#lang racket
;; λ-1 translator — Racket prelude

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
