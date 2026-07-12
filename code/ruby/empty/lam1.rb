# λ-1 translator — Ruby prelude
$failures = 0

def encodeInt(n)               # host int -> チャーチ数
  ->(f) { ->(x) { acc = x; n.times { acc = f.(acc) }; acc } }
end

def decodeInt(t)               # チャーチ数 -> 文字列
  t.(->(k) { k + 1 }).(0).to_s
end

def decodeBool(t)              # チャーチ真偽値 -> "true"/"false"
  t.('true').('false')
end

def _check(a, b, label)
  if a == b
    puts "ok   #{label}"
  else
    $failures += 1
    puts "FAIL #{label}: #{a.inspect} != #{b.inspect}"
  end
end

def _finish
  if $failures > 0
    puts "#{$failures} failure(s)"
    exit 1
  end
  puts "all green"
end

# ============ JSON 層（型付き値。ADR-0005） ============
def _mkpair(a, b)
  ->(s) { s.(a).(b) }
end
def _fstp(p)
  p.(->(a) { ->(b) { a } })
end
def _sndp(p)
  p.(->(a) { ->(b) { b } })
end
def _churchToInt(c)
  c.(->(k) { k + 1 }).(0)
end
def _boolToHost(c)
  c.(true).(false)
end
def _consH(h, t)
  ->(n) { ->(c) { c.(h).(t) } }
end
def _isNil(lst)
  _boolToHost(lst.(->(t) { ->(f) { t } }).(->(h) { ->(t) { ->(tt) { ->(ff) { ff } } } }))
end
def _headL(lst)
  lst.('').(->(h) { ->(t) { h } })
end
def _tailL(lst)
  lst.('').(->(h) { ->(t) { t } })
end
def _walk(lst)
  out = []
  until _isNil(lst)
    out << _headL(lst)
    lst = _tailL(lst)
  end
  out
end

def jInt(n)  ; _mkpair(encodeInt(1), encodeInt(n)) ; end
def jBool(b) ; _mkpair(encodeInt(2), b) ; end
def jStr(s)
  lst = ->(n) { ->(c) { n } }
  s.bytes.reverse_each { |byte| lst = _consH(encodeInt(byte), lst) }
  _mkpair(encodeInt(3), lst)
end
def jArr(lst) ; _mkpair(encodeInt(4), lst) ; end
def jObj(lst) ; _mkpair(encodeInt(5), lst) ; end
def jNull     ; _mkpair(encodeInt(6), ->(x) { x }) ; end

def _jsonEscape(s)
  out = '"'
  s.each_char do |c|
    if c == '"'
      out += '\\"'
    elsif c == '\\'
      out += '\\\\'
    else
      out += c
    end
  end
  out + '"'
end

def decodeJson(v)
  tag = _churchToInt(_fstp(v))
  payload = _sndp(v)
  case tag
  when 1 then _churchToInt(payload).to_s
  when 2 then _boolToHost(payload) ? 'true' : 'false'
  when 3
    bytes = _walk(payload).map { |b| _churchToInt(b) }
    _jsonEscape(bytes.pack('C*').force_encoding('UTF-8'))
  when 4 then '[' + _walk(payload).map { |x| decodeJson(x) }.join(',') + ']'
  when 5 then '{' + _walk(payload).map { |pr| decodeJson(_fstp(pr)) + ':' + decodeJson(_sndp(pr)) }.join(',') + '}'
  when 6 then 'null'
  else '?'
  end
end
