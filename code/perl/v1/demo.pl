use strict;
use warnings;
use FindBin;
require "$FindBin::Bin/lam1.pl";  # ヘルパーは lam1.pl

# --- definitions ---
my $_I = sub { my ($_x) = @_; $_x };
my $_K = sub { my ($_x) = @_; sub { my ($_y) = @_; $_x } };
my $_S = sub { my ($_x) = @_; sub { my ($_y) = @_; sub { my ($_z) = @_; $_x->($_z)->($_y->($_z)) } } };
my $_zero = sub { my ($_f) = @_; sub { my ($_x) = @_; $_x } };
my $_succ = sub { my ($_n) = @_; sub { my ($_f) = @_; sub { my ($_x) = @_; $_f->($_n->($_f)->($_x)) } } };
my $_add = sub { my ($_m) = @_; sub { my ($_n) = @_; sub { my ($_f) = @_; sub { my ($_x) = @_; $_m->($_f)->($_n->($_f)->($_x)) } } } };
my $_true = sub { my ($_t) = @_; sub { my ($_f) = @_; $_t } };
my $_false = sub { my ($_t) = @_; sub { my ($_f) = @_; $_f } };
my $_if = sub { my ($_b) = @_; sub { my ($_t) = @_; sub { my ($_e) = @_; $_b->($_t)->($_e) } } };
my $_and = sub { my ($_p) = @_; sub { my ($_q) = @_; $_p->($_q)->($_p) } };
my $_not = sub { my ($_b) = @_; $_b->($_false)->($_true) };

# --- assertions ---
_check('1', decodeInt($_S->($_K)->($_K)->(encodeInt(1))), 'assert 1');
_check('0', decodeInt($_zero), 'assert 2');
_check('3', decodeInt($_succ->(encodeInt(2))), 'assert 3');
_check('2', decodeInt($_add->(encodeInt(1))->(encodeInt(1))), 'assert 4');
_check('true', decodeBool($_and->($_true)->($_true)), 'assert 5');
_check('false', decodeBool($_and->($_true)->($_false)), 'assert 6');
_check('false', decodeBool($_if->($_false)->($_true)->($_false)), 'assert 7');
_check('true', decodeBool($_not->($_false)), 'assert 8');

_finish();
