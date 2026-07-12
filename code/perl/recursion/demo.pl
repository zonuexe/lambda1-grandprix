use strict;
use warnings;
use FindBin;
require "$FindBin::Bin/lam1.pl";  # ヘルパーは lam1.pl

# --- definitions ---
my $_Z = sub { my ($_f) = @_; sub { my ($_x) = @_; $_f->(sub { my ($_v) = @_; $_x->($_x)->($_v) }) }->(sub { my ($_x) = @_; $_f->(sub { my ($_v) = @_; $_x->($_x)->($_v) }) }) };
my $_one = sub { my ($_f) = @_; sub { my ($_x) = @_; $_f->($_x) } };
my $_mult = sub { my ($_m) = @_; sub { my ($_n) = @_; sub { my ($_f) = @_; $_m->($_n->($_f)) } } };
my $_pred = sub { my ($_n) = @_; sub { my ($_f) = @_; sub { my ($_x) = @_; $_n->(sub { my ($_g) = @_; sub { my ($_h) = @_; $_h->($_g->($_f)) } })->(sub { my ($_u) = @_; $_x })->(sub { my ($_u) = @_; $_u }) } } };
my $_true = sub { my ($_t) = @_; sub { my ($_f) = @_; $_t } };
my $_false = sub { my ($_t) = @_; sub { my ($_f) = @_; $_f } };
my $_isZero = sub { my ($_n) = @_; $_n->(sub { my ($_x) = @_; $_false })->($_true) };
my $_fstep = sub { my ($_rec) = @_; sub { my ($_n) = @_; $_isZero->($_n)->(sub { my ($_u) = @_; $_one })->(sub { my ($_u) = @_; $_mult->($_n)->($_rec->($_pred->($_n))) })->($_n) } };
my $_fact = $_Z->($_fstep);

# --- assertions ---
_check('1', decodeInt($_fact->(encodeInt(0))), 'assert 1');
_check('1', decodeInt($_fact->(encodeInt(1))), 'assert 2');
_check('2', decodeInt($_fact->(encodeInt(2))), 'assert 3');
_check('6', decodeInt($_fact->(encodeInt(3))), 'assert 4');
_check('120', decodeInt($_fact->(encodeInt(5))), 'assert 5');

_finish();
