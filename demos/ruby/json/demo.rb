require_relative 'lam1'  # ヘルパーは lam1.rb

# --- definitions ---
pair = (->(a) { (->(b) { (->(s) { s.(a).(b) }) }) })
_nil = (->(n) { (->(c) { n }) })
cons = (->(h) { (->(t) { (->(n) { (->(c) { c.(h).(t) }) }) }) })
_true = (->(t) { (->(f) { t }) })
_false = (->(t) { (->(f) { f }) })
snd = (->(p) { p.(_false) })
one = (->(f) { (->(x) { f.(x) }) })
pred = (->(n) { (->(f) { (->(x) { n.((->(g) { (->(h) { h.(g.(f)) }) })).((->(u) { x })).((->(u) { u })) }) }) })
tint = (->(k) { pair.(one).(k) })
step = (->(p) { p.((->(k) { (->(l) { pair.(pred.(k)).(cons.(tint.(k)).(l)) }) })) })
range = (->(n) { snd.(n.(step).(pair.(n).(_nil))) })

# --- assertions ---
_check('1', decodeJson(jInt(1)), 'assert 1')
_check('true', decodeJson(jBool(_true)), 'assert 2')
_check('false', decodeJson(jBool(_false)), 'assert 3')
_check('"hi"', decodeJson(jStr('hi')), 'assert 4')
_check('null', decodeJson(jNull()), 'assert 5')
_check('[1,true]', decodeJson(jArr(cons.(jInt(1)).(cons.(jBool(_true)).(_nil)))), 'assert 6')
_check('{"k":1}', decodeJson(jObj(cons.(pair.(jStr('k')).(jInt(1))).(_nil))), 'assert 7')
_check('[1,[2,3]]', decodeJson(jArr(cons.(jInt(1)).(cons.(jArr(cons.(jInt(2)).(cons.(jInt(3)).(_nil)))).(_nil)))), 'assert 8')
_check('[]', decodeJson(jArr(range.(encodeInt(0)))), 'assert 9')
_check('[1]', decodeJson(jArr(range.(encodeInt(1)))), 'assert 10')
_check('[1,2,3]', decodeJson(jArr(range.(encodeInt(3)))), 'assert 11')

_finish
