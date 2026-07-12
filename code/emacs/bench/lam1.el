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


;;; ============ JSON 層（型付き値。ADR-0005） ============
;;; Lisp-2 ゆえ λ値の適用は funcall、値として渡す関数は #'name。
(defun jK (a) (lambda (b) a))
(defun jKI (a) (lambda (b) b))
(defun mkpair (a b) (lambda (s) (funcall (funcall s a) b)))
(defun fstp (p) (funcall p #'jK))
(defun sndp (p) (funcall p #'jKI))
(defun churchToInt (c) (funcall (funcall c (lambda (k) (1+ k))) 0))
(defun jcTrue (tt) (lambda (f) tt))
(defun jcFalse (tt) (lambda (f) f))
(defun boolToHost (c) (funcall (funcall c t) nil))
(defvar nilH (lambda (n) (lambda (c) n)))
(defun consH (h tl) (lambda (n) (lambda (c) (funcall (funcall c h) tl))))
(defun isNil (lst)
  (boolToHost (funcall (funcall lst #'jcTrue)
                       (lambda (h) (lambda (tl) #'jcFalse)))))
(defun headL (lst)
  (funcall (funcall lst nil) (lambda (h) (lambda (tl) h))))
(defun tailL (lst)
  (funcall (funcall lst nil) (lambda (h) (lambda (tl) tl))))
(defun walk (lst)
  (let ((acc '()))
    (while (not (isNil lst))
      (setq acc (cons (headL lst) acc))
      (setq lst (tailL lst)))
    (nreverse acc)))

(defun jInt (n) (mkpair (encodeInt 1) (encodeInt n)))
(defun jBool (b) (mkpair (encodeInt 2) b))
(defun jStr (s)
  (mkpair (encodeInt 3)
          (let ((lst nilH))
            (dolist (c (nreverse (string-to-list s)) lst)
              (setq lst (consH (encodeInt c) lst))))))
(defun jArr (lst) (mkpair (encodeInt 4) lst))
(defun jObj (lst) (mkpair (encodeInt 5) lst))
(defun jNull () (mkpair (encodeInt 6) (lambda (x) x)))

(defun jsonEscape (s)
  (concat "\""
          (mapconcat (lambda (c)
                       (cond ((= c ?\") "\\\"")
                             ((= c ?\\) "\\\\")
                             (t (char-to-string c))))
                     s "")
          "\""))

(defun decodeJson (v)
  (let ((tag (churchToInt (fstp v)))
        (payload (sndp v)))
    (cond
     ((= tag 1) (number-to-string (churchToInt payload)))
     ((= tag 2) (if (boolToHost payload) "true" "false"))
     ((= tag 3) (jsonEscape (apply #'string (mapcar #'churchToInt (walk payload)))))
     ((= tag 4) (concat "[" (mapconcat #'decodeJson (walk payload) ",") "]"))
     ((= tag 5) (concat "{"
                        (mapconcat (lambda (pr)
                                     (concat (decodeJson (fstp pr)) ":" (decodeJson (sndp pr))))
                                   (walk payload) ",")
                        "}"))
     ((= tag 6) "null")
     (t "?"))))
