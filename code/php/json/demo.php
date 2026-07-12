<?php
require __DIR__ . '/lam1.php';  // ヘルパーは lam1.php

// --- definitions ---
$pair = (fn($a) => (fn($b) => (fn($s) => (($s)($a))($b))));
$nil = (fn($n) => (fn($c) => $n));
$cons = (fn($h) => (fn($t) => (fn($n) => (fn($c) => (($c)($h))($t)))));
$true = (fn($t) => (fn($f) => $t));
$false = (fn($t) => (fn($f) => $f));
$snd = (fn($p) => ($p)($false));
$one = (fn($f) => (fn($x) => ($f)($x)));
$pred = (fn($n) => (fn($f) => (fn($x) => ((($n)((fn($g) => (fn($h) => ($h)(($g)($f))))))((fn($u) => $x)))((fn($u) => $u)))));
$tint = (fn($k) => (($pair)($one))($k));
$step = (fn($p) => ($p)((fn($k) => (fn($l) => (($pair)(($pred)($k)))((($cons)(($tint)($k)))($l))))));
$range = (fn($n) => ($snd)((($n)($step))((($pair)($n))($nil))));

// --- assertions ---
_check('1', decodeJson(jInt(1)), 'assert 1');
_check('true', decodeJson(jBool($true)), 'assert 2');
_check('false', decodeJson(jBool($false)), 'assert 3');
_check('"hi"', decodeJson(jStr('hi')), 'assert 4');
_check('null', decodeJson(jNull()), 'assert 5');
_check('[1,true]', decodeJson(jArr((($cons)(jInt(1)))((($cons)(jBool($true)))($nil)))), 'assert 6');
_check('{"k":1}', decodeJson(jObj((($cons)((($pair)(jStr('k')))(jInt(1))))($nil))), 'assert 7');
_check('[1,[2,3]]', decodeJson(jArr((($cons)(jInt(1)))((($cons)(jArr((($cons)(jInt(2)))((($cons)(jInt(3)))($nil)))))($nil)))), 'assert 8');
_check('[]', decodeJson(jArr(($range)(encodeInt(0)))), 'assert 9');
_check('[1]', decodeJson(jArr(($range)(encodeInt(1)))), 'assert 10');
_check('[1,2,3]', decodeJson(jArr(($range)(encodeInt(3)))), 'assert 11');

_finish();
