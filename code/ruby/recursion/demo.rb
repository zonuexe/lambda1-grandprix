require_relative 'lam1'  # ヘルパーは lam1.rb

# --- definitions ---
Z = (->(f) { (->(x) { f.((->(v) { x.(x).(v) })) }).((->(x) { f.((->(v) { x.(x).(v) })) })) })
one = (->(f) { (->(x) { f.(x) }) })
mult = (->(m) { (->(n) { (->(f) { m.(n.(f)) }) }) })
pred = (->(n) { (->(f) { (->(x) { n.((->(g) { (->(h) { h.(g.(f)) }) })).((->(u) { x })).((->(u) { u })) }) }) })
_true = (->(t) { (->(f) { t }) })
_false = (->(t) { (->(f) { f }) })
isZero = (->(n) { n.((->(x) { _false })).(_true) })
fstep = (->(rec) { (->(n) { isZero.(n).((->(u) { one })).((->(u) { mult.(n).(rec.(pred.(n))) })).(n) }) })
fact = Z.(fstep)

# --- assertions ---
_check('1', decodeInt(fact.(encodeInt(0))), 'assert 1')
_check('1', decodeInt(fact.(encodeInt(1))), 'assert 2')
_check('2', decodeInt(fact.(encodeInt(2))), 'assert 3')
_check('6', decodeInt(fact.(encodeInt(3))), 'assert 4')
_check('120', decodeInt(fact.(encodeInt(5))), 'assert 5')

_finish
