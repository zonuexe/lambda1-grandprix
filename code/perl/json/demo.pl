use strict;
use warnings;
use FindBin;
require "$FindBin::Bin/lam1.pl";  # ヘルパーは lam1.pl

# --- definitions ---
my $_pair = sub { my ($_a) = @_; sub { my ($_b) = @_; sub { my ($_s) = @_; $_s->($_a)->($_b) } } };
my $_nil = sub { my ($_n) = @_; sub { my ($_c) = @_; $_n } };
my $_cons = sub { my ($_h) = @_; sub { my ($_t) = @_; sub { my ($_n) = @_; sub { my ($_c) = @_; $_c->($_h)->($_t) } } } };
my $_true = sub { my ($_t) = @_; sub { my ($_f) = @_; $_t } };
my $_false = sub { my ($_t) = @_; sub { my ($_f) = @_; $_f } };
my $_snd = sub { my ($_p) = @_; $_p->($_false) };
my $_one = sub { my ($_f) = @_; sub { my ($_x) = @_; $_f->($_x) } };
my $_pred = sub { my ($_n) = @_; sub { my ($_f) = @_; sub { my ($_x) = @_; $_n->(sub { my ($_g) = @_; sub { my ($_h) = @_; $_h->($_g->($_f)) } })->(sub { my ($_u) = @_; $_x })->(sub { my ($_u) = @_; $_u }) } } };
my $_tint = sub { my ($_k) = @_; $_pair->($_one)->($_k) };
my $_step = sub { my ($_p) = @_; $_p->(sub { my ($_k) = @_; sub { my ($_l) = @_; $_pair->($_pred->($_k))->($_cons->($_tint->($_k))->($_l)) } }) };
my $_range = sub { my ($_n) = @_; $_snd->($_n->($_step)->($_pair->($_n)->($_nil))) };

# --- assertions ---
_check('1', decodeJson(jInt(1)), 'assert 1');
_check('true', decodeJson(jBool($_true)), 'assert 2');
_check('false', decodeJson(jBool($_false)), 'assert 3');
_check('"hi"', decodeJson(jStr('hi')), 'assert 4');
_check('null', decodeJson(jNull()), 'assert 5');
_check('[1,true]', decodeJson(jArr($_cons->(jInt(1))->($_cons->(jBool($_true))->($_nil)))), 'assert 6');
_check('{"k":1}', decodeJson(jObj($_cons->($_pair->(jStr('k'))->(jInt(1)))->($_nil))), 'assert 7');
_check('[1,[2,3]]', decodeJson(jArr($_cons->(jInt(1))->($_cons->(jArr($_cons->(jInt(2))->($_cons->(jInt(3))->($_nil))))->($_nil)))), 'assert 8');
_check('[]', decodeJson(jArr($_range->(encodeInt(0)))), 'assert 9');
_check('[1]', decodeJson(jArr($_range->(encodeInt(1)))), 'assert 10');
_check('[1,2,3]', decodeJson(jArr($_range->(encodeInt(3)))), 'assert 11');

_finish();
