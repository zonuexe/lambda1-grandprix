;;; -*- lexical-binding: t; -*-
;;; λ-1 translator — Emacs Lisp prelude

(defvar _failures 0)

(defun encodeInt (n)             ; host int -> チャーチ数
  (lambda (f)
    (lambda (x)
      (let ((acc x) (i 0))
        (while (< i n)
          (setq acc (funcall f acc))
          (setq i (1+ i)))
        acc))))

(defun decodeInt (v)             ; チャーチ数 -> 文字列（`t` は真値なので使わない）
  (number-to-string (funcall (funcall v (lambda (k) (1+ k))) 0)))

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
