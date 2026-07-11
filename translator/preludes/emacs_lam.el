;;; -*- lexical-binding: t; -*-
;;; λ-1 translator — Emacs Lisp prelude（λ マクロ変種）

;; Emacs Lisp に λ 構文は無いが、マクロで自作できる（メタプログラミング）:
;;   (λ x body) => (lambda (x) body)
;; ※ (λ x . body) の「.」は Lisp リーダのドット対と衝突し入れ子が壊れるため使えない。
(defmacro λ (arg &rest body)
  `(lambda (,arg) ,@body))

(defvar _failures 0)

(defun encodeInt (n)             ; host int -> チャーチ数
  (λ f (λ x
    (let ((acc x) (i 0))
      (while (< i n)
        (setq acc (funcall f acc))
        (setq i (1+ i)))
      acc))))

(defun decodeInt (v)             ; チャーチ数 -> 文字列（`t` は真値なので使わない）
  (number-to-string (funcall (funcall v (λ k (1+ k))) 0)))

(defun decodeBool (v)            ; チャーチ真偽値 -> "true"/"false"
  (funcall (funcall v "true") "false"))

(defun _check (a b label)
  (if (equal a b)
      (princ (format "ok   %s\n" label))
    (progn
      (setq _failures (1+ _failures))
      (princ (format "FAIL %s: %S != %S\n" label a b)))))

(defun _finish ()
  (when (> _failures 0)
    (princ (format "%d failure(s)\n" _failures))
    (kill-emacs 1))
  (princ "all green\n"))
