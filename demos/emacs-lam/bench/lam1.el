;;; -*- lexical-binding: t; -*-
;;; λ-1 translator — Emacs Lisp prelude（λ マクロ変種）

;; Emacs Lisp に λ 構文は無いが、マクロで自作できる（メタプログラミング）:
;;   (λ x body) => (lambda (x) body)
;; ※ (λ x . body) の「.」は Lisp リーダのドット対と衝突し入れ子が壊れるため使えない。
(defmacro λ (arg &rest body)
  `(lambda (,arg) ,@body))

;; 左結合の関数適用マクロ（メタプログラミング）:
;;   ($ f a b c) => (funcall (funcall (funcall f a) b) c)
;; Lisp-2 なので適用は funcall が要る。並置 (f a b c) と同じ見た目に畳んで
;; funcall の入れ子ノイズを消す。
(defmacro $ (fn &rest args)
  (if (null args)
      fn
    `($ (funcall ,fn ,(car args)) ,@(cdr args))))

(defvar _failures 0)

(defun encodeInt (n)             ; host int -> チャーチ数
  (λ f (λ x
    (let ((acc x) (i 0))
      (while (< i n)
        (setq acc ($ f acc))
        (setq i (1+ i)))
      acc))))

(defun decodeInt (v)             ; チャーチ数 -> 文字列（`t` は真値なので使わない）
  (number-to-string ($ v (λ k (1+ k)) 0)))

(defun decodeBool (v)            ; チャーチ真偽値 -> "true"/"false"
  ($ v "true" "false"))

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
;;; 単引数ラムダは自作 λ マクロ。適用は $ マクロ、値渡しの関数は #'name。
(defun jK (a) (λ b a))
(defun jKI (a) (λ b b))
(defun mkpair (a b) (λ s ($ s a b)))
(defun fstp (p) ($ p #'jK))
(defun sndp (p) ($ p #'jKI))
(defun churchToInt (c) ($ c (λ k (1+ k)) 0))
(defun jcTrue (tt) (λ f tt))
(defun jcFalse (tt) (λ f f))
(defun boolToHost (c) ($ c t nil))
(defvar nilH (λ n (λ c n)))
(defun consH (h tl) (λ n (λ c ($ c h tl))))
(defun isNil (lst)
  (boolToHost ($ lst #'jcTrue
                 (λ h (λ tl #'jcFalse)))))
(defun headL (lst)
  ($ lst nil (λ h (λ tl h))))
(defun tailL (lst)
  ($ lst nil (λ h (λ tl tl))))
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
(defun jNull () (mkpair (encodeInt 6) (λ x x)))

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
