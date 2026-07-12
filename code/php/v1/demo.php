<?php
require __DIR__ . '/lam1.php';  // ヘルパーは lam1.php

// --- definitions ---
$I = (fn($x) => $x);
$K = (fn($x) => (fn($y) => $x));
$S = (fn($x) => (fn($y) => (fn($z) => (($x)($z))(($y)($z)))));
$zero = (fn($f) => (fn($x) => $x));
$succ = (fn($n) => (fn($f) => (fn($x) => ($f)((($n)($f))($x)))));
$add = (fn($m) => (fn($n) => (fn($f) => (fn($x) => (($m)($f))((($n)($f))($x))))));
$true = (fn($t) => (fn($f) => $t));
$false = (fn($t) => (fn($f) => $f));
$if = (fn($b) => (fn($t) => (fn($e) => (($b)($t))($e))));
$and = (fn($p) => (fn($q) => (($p)($q))($p)));
$not = (fn($b) => (($b)($false))($true));

// --- assertions ---
_check('1', decodeInt(((($S)($K))($K))(encodeInt(1))), 'assert 1');
_check('0', decodeInt($zero), 'assert 2');
_check('3', decodeInt(($succ)(encodeInt(2))), 'assert 3');
_check('2', decodeInt((($add)(encodeInt(1)))(encodeInt(1))), 'assert 4');
_check('true', decodeBool((($and)($true))($true)), 'assert 5');
_check('false', decodeBool((($and)($true))($false)), 'assert 6');
_check('false', decodeBool(((($if)($false))($true))($false)), 'assert 7');
_check('true', decodeBool(($not)($false)), 'assert 8');

_finish();
