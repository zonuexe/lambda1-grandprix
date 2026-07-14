require_relative 'lam1'  # ヘルパーは lam1.rb

# --- definitions ---
I = (->(x) { x })
K = (->(x) { (->(y) { x }) })
S = (->(x) { (->(y) { (->(z) { x.(z).(y.(z)) }) }) })
zero = (->(f) { (->(x) { x }) })
succ = (->(n) { (->(f) { (->(x) { f.(n.(f).(x)) }) }) })
add = (->(m) { (->(n) { (->(f) { (->(x) { m.(f).(n.(f).(x)) }) }) }) })
_true = (->(t) { (->(f) { t }) })
_false = (->(t) { (->(f) { f }) })
_if = (->(b) { (->(t) { (->(e) { b.(t).(e) }) }) })
_and = (->(p) { (->(q) { p.(q).(p) }) })
_not = (->(b) { b.(_false).(_true) })

# --- assertions ---
_check('1', decodeInt(S.(K).(K).(encodeInt(1))), 'assert 1')
_check('0', decodeInt(zero), 'assert 2')
_check('3', decodeInt(succ.(encodeInt(2))), 'assert 3')
_check('2', decodeInt(add.(encodeInt(1)).(encodeInt(1))), 'assert 4')
_check('true', decodeBool(_and.(_true).(_true)), 'assert 5')
_check('false', decodeBool(_and.(_true).(_false)), 'assert 6')
_check('false', decodeBool(_if.(_false).(_true).(_false)), 'assert 7')
_check('true', decodeBool(_not.(_false)), 'assert 8')

_finish
