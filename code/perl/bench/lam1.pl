use strict;
use warnings;
# λ-1 translator — Perl prelude
my $failures = 0;

sub encodeInt {                    # host int -> チャーチ数
    my ($n) = @_;
    return sub {
        my ($f) = @_;
        return sub {
            my ($x) = @_;
            my $acc = $x;
            for (1 .. $n) { $acc = $f->($acc); }
            return $acc;
        };
    };
}

sub decodeInt {                    # チャーチ数 -> 文字列
    my ($t) = @_;
    return "" . $t->(sub { $_[0] + 1 })->(0);
}

sub decodeBool {                   # チャーチ真偽値 -> "true"/"false"
    my ($t) = @_;
    return $t->('true')->('false');
}

sub _check {
    my ($a, $b, $label) = @_;
    if ($a eq $b) {
        print "ok   $label\n";
    } else {
        $failures++;
        print "FAIL $label: $a != $b\n";
    }
}

sub _finish {
    if ($failures > 0) {
        print "$failures failure(s)\n";
        exit 1;
    }
    print "all green\n";
}

# ============ JSON 層（型付き値。ADR-0005） ============
sub _mkpair {
    my ($a, $b) = @_;
    return sub { my ($s) = @_; return $s->($a)->($b); };
}
sub _fstp {
    my ($p) = @_;
    return $p->(sub { my ($a) = @_; return sub { my ($b) = @_; return $a; }; });
}
sub _sndp {
    my ($p) = @_;
    return $p->(sub { my ($a) = @_; return sub { my ($b) = @_; return $b; }; });
}
sub _churchToInt {
    my ($c) = @_;
    return $c->(sub { $_[0] + 1 })->(0);
}
sub _boolToHost {
    my ($c) = @_;
    return $c->(1)->(0);
}
sub _consH {
    my ($h, $t) = @_;
    return sub { my ($n) = @_; return sub { my ($c) = @_; return $c->($h)->($t); }; };
}
sub _isNil {
    my ($lst) = @_;
    return _boolToHost(
        $lst->(sub { my ($t) = @_; return sub { my ($f) = @_; return $t; }; })
            ->(sub { my ($h) = @_; return sub { my ($t) = @_;
                return sub { my ($tt) = @_; return sub { my ($ff) = @_; return $ff; }; }; }; })
    );
}
sub _headL {
    my ($lst) = @_;
    return $lst->('')->(sub { my ($h) = @_; return sub { my ($t) = @_; return $h; }; });
}
sub _tailL {
    my ($lst) = @_;
    return $lst->('')->(sub { my ($h) = @_; return sub { my ($t) = @_; return $t; }; });
}
sub _walk {
    my ($lst) = @_;
    my @out;
    while (!_isNil($lst)) {
        push @out, _headL($lst);
        $lst = _tailL($lst);
    }
    return @out;
}

sub jInt  { my ($n) = @_; return _mkpair(encodeInt(1), encodeInt($n)); }
sub jBool { my ($b) = @_; return _mkpair(encodeInt(2), $b); }
sub jStr {
    my ($s) = @_;
    my $lst = sub { my ($n) = @_; return sub { my ($c) = @_; return $n; }; };
    my @bytes = unpack('C*', $s);
    for my $byte (reverse @bytes) {
        $lst = _consH(encodeInt($byte), $lst);
    }
    return _mkpair(encodeInt(3), $lst);
}
sub jArr  { my ($lst) = @_; return _mkpair(encodeInt(4), $lst); }
sub jObj  { my ($lst) = @_; return _mkpair(encodeInt(5), $lst); }
sub jNull { return _mkpair(encodeInt(6), sub { my ($x) = @_; return $x; }); }

sub _jsonEscape {
    my ($s) = @_;
    my $out = '"';
    for my $ch (split //, $s) {
        if ($ch eq '"') {
            $out .= '\\"';
        } elsif ($ch eq '\\') {
            $out .= '\\\\';
        } else {
            $out .= $ch;
        }
    }
    return $out . '"';
}

sub decodeJson {
    my ($v) = @_;
    my $tag = _churchToInt(_fstp($v));
    my $payload = _sndp($v);
    if ($tag == 1) {
        return "" . _churchToInt($payload);
    } elsif ($tag == 2) {
        return _boolToHost($payload) ? 'true' : 'false';
    } elsif ($tag == 3) {
        my @bytes = map { _churchToInt($_) } _walk($payload);
        return _jsonEscape(pack('C*', @bytes));
    } elsif ($tag == 4) {
        return '[' . join(',', map { decodeJson($_) } _walk($payload)) . ']';
    } elsif ($tag == 5) {
        return '{' . join(',', map { decodeJson(_fstp($_)) . ':' . decodeJson(_sndp($_)) } _walk($payload)) . '}';
    } elsif ($tag == 6) {
        return 'null';
    } else {
        return '?';
    }
}

1;  # require が真を要求するため
