<?php
require __DIR__ . '/lam1.php';  // ヘルパーは lam1.php

// --- definitions ---
$_pair = (fn($_a) => (fn($_b) => (fn($_s) => (($_s)($_a))($_b))));
$_nil = (fn($_n) => (fn($_c) => $_n));
$_cons = (fn($_h) => (fn($_t) => (fn($_n) => (fn($_c) => (($_c)($_h))($_t)))));
$_true = (fn($_t) => (fn($_f) => $_t));
$_false = (fn($_t) => (fn($_f) => $_f));
$_snd = (fn($_p) => ($_p)($_false));
$_one = (fn($_f) => (fn($_x) => ($_f)($_x)));
$_pred = (fn($_n) => (fn($_f) => (fn($_x) => ((($_n)((fn($_g) => (fn($_h) => ($_h)(($_g)($_f))))))((fn($_u) => $_x)))((fn($_u) => $_u)))));
$_tint = (fn($_k) => (($_pair)($_one))($_k));
$_step = (fn($_p) => ($_p)((fn($_k) => (fn($_l) => (($_pair)(($_pred)($_k)))((($_cons)(($_tint)($_k)))($_l))))));
$_range = (fn($_n) => ($_snd)((($_n)($_step))((($_pair)($_n))($_nil))));

// --- assertions ---
_check('1', decodeJson(jInt(1)), 'assert 1');
_check('true', decodeJson(jBool($_true)), 'assert 2');
_check('false', decodeJson(jBool($_false)), 'assert 3');
_check('"hi"', decodeJson(jStr('hi')), 'assert 4');
_check('null', decodeJson(jNull()), 'assert 5');
_check('[1,true]', decodeJson(jArr((($_cons)(jInt(1)))((($_cons)(jBool($_true)))($_nil)))), 'assert 6');
_check('{"k":1}', decodeJson(jObj((($_cons)((($_pair)(jStr('k')))(jInt(1))))($_nil))), 'assert 7');
_check('[1,[2,3]]', decodeJson(jArr((($_cons)(jInt(1)))((($_cons)(jArr((($_cons)(jInt(2)))((($_cons)(jInt(3)))($_nil)))))($_nil)))), 'assert 8');
_check('[]', decodeJson(jArr(($_range)(encodeInt(0)))), 'assert 9');
_check('[1]', decodeJson(jArr(($_range)(encodeInt(1)))), 'assert 10');
_check('[1,2,3]', decodeJson(jArr(($_range)(encodeInt(3)))), 'assert 11');

_finish();
