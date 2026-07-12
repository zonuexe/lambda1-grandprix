<?php
// λ-1 translator — PHP prelude
$GLOBALS['failures'] = 0;

function encodeInt($n) {            // host int -> チャーチ数
    return function ($f) use ($n) {
        return function ($x) use ($f, $n) {
            $acc = $x;
            for ($i = 0; $i < $n; $i++) { $acc = $f($acc); }
            return $acc;
        };
    };
}
function decodeInt($t) {            // チャーチ数 -> 文字列
    return (string)(($t(fn($k) => $k + 1))(0));
}
function decodeBool($t) {           // チャーチ真偽値 -> "true"/"false"
    return ($t('true'))('false');
}
function _check($a, $b, $label) {
    if ($a === $b) {
        echo "ok   $label\n";
    } else {
        $GLOBALS['failures']++;
        echo "FAIL $label: " . var_export($a, true) . " != " . var_export($b, true) . "\n";
    }
}
function _finish() {
    if ($GLOBALS['failures'] > 0) {
        echo $GLOBALS['failures'] . " failure(s)\n";
        exit(1);
    }
    echo "all green\n";
}

// ============ JSON 層（型付き値。ADR-0005） ============
function _mkpair($a, $b) {
    return fn($s) => ($s($a))($b);
}
function _fstp($p) {
    return $p(fn($a) => fn($b) => $a);
}
function _sndp($p) {
    return $p(fn($a) => fn($b) => $b);
}
function _churchToInt($c) {
    return ($c(fn($k) => $k + 1))(0);
}
function _boolToHost($c) {
    return ($c(true))(false);
}
function _consH($h, $t) {
    return fn($n) => fn($c) => ($c($h))($t);
}
function _isNil($lst) {
    return _boolToHost(
        ($lst(fn($t) => fn($f) => $t))
            (fn($h) => fn($t) => fn($tt) => fn($ff) => $ff)
    );
}
function _headL($lst) {
    return ($lst(''))(fn($h) => fn($t) => $h);
}
function _tailL($lst) {
    return ($lst(''))(fn($h) => fn($t) => $t);
}
function _walk($lst) {
    $out = [];
    while (!_isNil($lst)) {
        $out[] = _headL($lst);
        $lst = _tailL($lst);
    }
    return $out;
}

function jInt($n)  { return _mkpair(encodeInt(1), encodeInt($n)); }
function jBool($b) { return _mkpair(encodeInt(2), $b); }
function jStr($s) {
    $lst = fn($n) => fn($c) => $n;
    $bytes = array_values(unpack('C*', $s));
    for ($i = count($bytes) - 1; $i >= 0; $i--) {
        $lst = _consH(encodeInt($bytes[$i]), $lst);
    }
    return _mkpair(encodeInt(3), $lst);
}
function jArr($lst) { return _mkpair(encodeInt(4), $lst); }
function jObj($lst) { return _mkpair(encodeInt(5), $lst); }
function jNull()    { return _mkpair(encodeInt(6), fn($x) => $x); }

function _jsonEscape($s) {
    $out = '"';
    $len = strlen($s);
    for ($i = 0; $i < $len; $i++) {
        $ch = $s[$i];
        if ($ch === '"') {
            $out .= '\\"';
        } elseif ($ch === '\\') {
            $out .= '\\\\';
        } else {
            $out .= $ch;
        }
    }
    return $out . '"';
}

function decodeJson($v) {
    $tag = _churchToInt(_fstp($v));
    $payload = _sndp($v);
    switch ($tag) {
        case 1: return (string) _churchToInt($payload);
        case 2: return _boolToHost($payload) ? 'true' : 'false';
        case 3:
            $str = '';
            foreach (_walk($payload) as $b) { $str .= chr(_churchToInt($b)); }
            return _jsonEscape($str);
        case 4:
            return '[' . implode(',', array_map('decodeJson', _walk($payload))) . ']';
        case 5:
            $parts = [];
            foreach (_walk($payload) as $pr) {
                $parts[] = decodeJson(_fstp($pr)) . ':' . decodeJson(_sndp($pr));
            }
            return '{' . implode(',', $parts) . '}';
        case 6: return 'null';
        default: return '?';
    }
}
