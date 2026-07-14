# 生成コード（聴衆の目視用）

`translator/scripts/gen-all.sh` が出力する、全コーパス × 全言語の生成物。
`demos/<言語>/<コーパス>/` に自己完結で配置（demo.<ext> ＋ 分離ライブラリ ＋ source.lam）。
聴衆は自分の言語のディレクトリだけを単独で読めます。

| 言語 | bench | empty | json | recursion | v1 |
| --- | --- | --- | --- | --- | --- |
| clojure-lam | [demo.clj](clojure-lam/bench/) | [demo.clj](clojure-lam/empty/) | [demo.clj](clojure-lam/json/) | [demo.clj](clojure-lam/recursion/) | [demo.clj](clojure-lam/v1/) |
| clojure | [demo.clj](clojure/bench/) | [demo.clj](clojure/empty/) | [demo.clj](clojure/json/) | [demo.clj](clojure/recursion/) | [demo.clj](clojure/v1/) |
| cpp | [demo.cpp](cpp/bench/) | [demo.cpp](cpp/empty/) | [demo.cpp](cpp/json/) | [demo.cpp](cpp/recursion/) | [demo.cpp](cpp/v1/) |
| emacs-lam | [demo.el](emacs-lam/bench/) | [demo.el](emacs-lam/empty/) | [demo.el](emacs-lam/json/) | [demo.el](emacs-lam/recursion/) | [demo.el](emacs-lam/v1/) |
| emacs | [demo.el](emacs/bench/) | [demo.el](emacs/empty/) | [demo.el](emacs/json/) | [demo.el](emacs/recursion/) | [demo.el](emacs/v1/) |
| go | [demo.go](go/bench/) | [demo.go](go/empty/) | [demo.go](go/json/) | [demo.go](go/recursion/) | [demo.go](go/v1/) |
| haskell | [demo.hs](haskell/bench/) | [demo.hs](haskell/empty/) | [demo.hs](haskell/json/) | [demo.hs](haskell/recursion/) | [demo.hs](haskell/v1/) |
| java | [demo.java](java/bench/) | [demo.java](java/empty/) | [demo.java](java/json/) | [demo.java](java/recursion/) | [demo.java](java/v1/) |
| kotlin | [demo.kt](kotlin/bench/) | [demo.kt](kotlin/empty/) | [demo.kt](kotlin/json/) | [demo.kt](kotlin/recursion/) | [demo.kt](kotlin/v1/) |
| lazyk | [demo.lazy](lazyk/bench/) | [demo.lazy](lazyk/empty/) | [demo.lazy](lazyk/json/) | [demo.lazy](lazyk/recursion/) | [demo.lazy](lazyk/v1/) |
| newlisp | [demo.lsp](newlisp/bench/) | [demo.lsp](newlisp/empty/) | [demo.lsp](newlisp/json/) | [demo.lsp](newlisp/recursion/) | [demo.lsp](newlisp/v1/) |
| pascal | [demo.pas](pascal/bench/) | [demo.pas](pascal/empty/) | [demo.pas](pascal/json/) | [demo.pas](pascal/recursion/) | [demo.pas](pascal/v1/) |
| perl | [demo.pl](perl/bench/) | [demo.pl](perl/empty/) | [demo.pl](perl/json/) | [demo.pl](perl/recursion/) | [demo.pl](perl/v1/) |
| php-ski | [demo.php](php-ski/bench/) | [demo.php](php-ski/empty/) | [demo.php](php-ski/json/) | [demo.php](php-ski/recursion/) | [demo.php](php-ski/v1/) |
| php | [demo.php](php/bench/) | [demo.php](php/empty/) | [demo.php](php/json/) | [demo.php](php/recursion/) | [demo.php](php/v1/) |
| python | [demo.py](python/bench/) | [demo.py](python/empty/) | [demo.py](python/json/) | [demo.py](python/recursion/) | [demo.py](python/v1/) |
| racket | [demo.rkt](racket/bench/) | [demo.rkt](racket/empty/) | [demo.rkt](racket/json/) | [demo.rkt](racket/recursion/) | [demo.rkt](racket/v1/) |
| ruby | [demo.rb](ruby/bench/) | [demo.rb](ruby/empty/) | [demo.rb](ruby/json/) | [demo.rb](ruby/recursion/) | [demo.rb](ruby/v1/) |
| rust | [demo.rs](rust/bench/) | [demo.rs](rust/empty/) | [demo.rs](rust/json/) | [demo.rs](rust/recursion/) | [demo.rs](rust/v1/) |
| scala-infix | [demo.scala](scala-infix/bench/) | [demo.scala](scala-infix/empty/) | [demo.scala](scala-infix/json/) | [demo.scala](scala-infix/recursion/) | [demo.scala](scala-infix/v1/) |
| scala | [demo.scala](scala/bench/) | [demo.scala](scala/empty/) | [demo.scala](scala/json/) | [demo.scala](scala/recursion/) | [demo.scala](scala/v1/) |
| sml | [demo.sml](sml/bench/) | [demo.sml](sml/empty/) | [demo.sml](sml/json/) | [demo.sml](sml/recursion/) | [demo.sml](sml/v1/) |
| swift | [demo.swift](swift/bench/) | [demo.swift](swift/empty/) | [demo.swift](swift/json/) | [demo.swift](swift/recursion/) | [demo.swift](swift/v1/) |
