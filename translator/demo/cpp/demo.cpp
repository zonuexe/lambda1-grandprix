// λ-1 translator — C++ prelude (system clang++)
#include <iostream>
#include <functional>
#include <string>
#include <variant>

struct D;
using Fn = std::function<D(D)>;

// 万能型（タグ付き union）: variant<Fn, long, string>。
struct D {
    std::variant<Fn, long, std::string> v;
    D(Fn f) : v(std::move(f)) {}
    D(long n) : v(n) {}
    D(std::string s) : v(std::move(s)) {}
    D apply(D x) const { return std::get<Fn>(v)(std::move(x)); }
};

D encodeInt(long n) {
    return D(Fn([n](D f) {
        return D(Fn([n, f](D x) {
            D acc = x;
            for (long i = 0; i < n; i++) acc = f.apply(acc);
            return acc;
        }));
    }));
}

std::string decodeInt(D t) {
    D incr = D(Fn([](D v) { return D(std::get<long>(v.v) + 1); }));
    D r = t.apply(incr).apply(D((long)0));
    return std::to_string(std::get<long>(r.v));
}

std::string decodeBool(D t) {
    D r = t.apply(D(std::string("true"))).apply(D(std::string("false")));
    return std::get<std::string>(r.v);
}

int _failures = 0;
void check(const std::string& label, const std::string& a, const std::string& b) {
    if (a == b) std::cout << "ok   " << label << "\n";
    else { _failures++; std::cout << "FAIL " << label << ": " << a << " != " << b << "\n"; }
}

D _I = D(Fn([](D _x) { return _x; }));
D _K = D(Fn([](D _x) { return D(Fn([_x](D _y) { return _x; })); }));
D _S = D(Fn([](D _x) { return D(Fn([_x](D _y) { return D(Fn([_x, _y](D _z) { return _x.apply(_z).apply(_y.apply(_z)); })); })); }));
D _zero = D(Fn([](D _f) { return D(Fn([](D _x) { return _x; })); }));
D _succ = D(Fn([](D _n) { return D(Fn([_n](D _f) { return D(Fn([_f, _n](D _x) { return _f.apply(_n.apply(_f).apply(_x)); })); })); }));
D _add = D(Fn([](D _m) { return D(Fn([_m](D _n) { return D(Fn([_m, _n](D _f) { return D(Fn([_f, _m, _n](D _x) { return _m.apply(_f).apply(_n.apply(_f).apply(_x)); })); })); })); }));
D _true = D(Fn([](D _t) { return D(Fn([_t](D _f) { return _t; })); }));
D _false = D(Fn([](D _t) { return D(Fn([](D _f) { return _f; })); }));
D _if = D(Fn([](D _b) { return D(Fn([_b](D _t) { return D(Fn([_b, _t](D _e) { return _b.apply(_t).apply(_e); })); })); }));
D _and = D(Fn([](D _p) { return D(Fn([_p](D _q) { return _p.apply(_q).apply(_p); })); }));
D _not = D(Fn([](D _b) { return _b.apply(_false).apply(_true); }));

int main() {
    check("assert 1", "1", decodeInt(_S.apply(_K).apply(_K).apply(encodeInt(1))));
    check("assert 2", "0", decodeInt(_zero));
    check("assert 3", "3", decodeInt(_succ.apply(encodeInt(2))));
    check("assert 4", "2", decodeInt(_add.apply(encodeInt(1)).apply(encodeInt(1))));
    check("assert 5", "true", decodeBool(_and.apply(_true).apply(_true)));
    check("assert 6", "false", decodeBool(_and.apply(_true).apply(_false)));
    check("assert 7", "false", decodeBool(_if.apply(_false).apply(_true).apply(_false)));
    check("assert 8", "true", decodeBool(_not.apply(_false)));
    if (_failures > 0) { std::cout << _failures << " failure(s)\n"; return 1; }
    std::cout << "all green\n";
    return 0;
}
