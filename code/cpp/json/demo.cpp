// λ-1 translator — C++ prelude (system clang++)
#include <iostream>
#include <functional>
#include <string>
#include <variant>
#include <vector>

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

// ============ JSON 層（型付き値。ADR-0005） ============
D selK()  { return D(Fn([](D a) { return D(Fn([a](D) { return a; })); })); }   // λa.λb.a
D selKI() { return D(Fn([](D)  { return D(Fn([](D b) { return b; })); })); }    // λa.λb.b
D cTrue()  { return D(Fn([](D t) { return D(Fn([t](D) { return t; })); })); }
D cFalse() { return D(Fn([](D)  { return D(Fn([](D f) { return f; })); })); }

D mkpair(D a, D b) { return D(Fn([a, b](D s) { return s.apply(a).apply(b); })); }
D fstp(D p) { return p.apply(selK()); }
D sndp(D p) { return p.apply(selKI()); }

long churchToInt(D c) {
    D incr = D(Fn([](D v) { return D(std::get<long>(v.v) + 1); }));
    return std::get<long>(c.apply(incr).apply(D((long)0)).v);
}
bool boolToHost(D c) {
    D r = c.apply(D(std::string("T"))).apply(D(std::string("F")));
    return std::get<std::string>(r.v) == "T";
}

D nilH() { return D(Fn([](D n) { return D(Fn([n](D) { return n; })); })); }
D consH(D h, D t) { return D(Fn([h, t](D) { return D(Fn([h, t](D c) { return c.apply(h).apply(t); })); })); }
bool isNil(D lst) {
    D consCase = D(Fn([](D) { return D(Fn([](D) { return cFalse(); })); }));
    return boolToHost(lst.apply(cTrue()).apply(consCase));
}
D headL(D lst) { return lst.apply(D(std::string(""))).apply(selK()); }
D tailL(D lst) { return lst.apply(D(std::string(""))).apply(selKI()); }
std::vector<D> walkL(D lst) {
    std::vector<D> out;
    while (!isNil(lst)) {
        out.push_back(headL(lst));
        lst = tailL(lst);
    }
    return out;
}

D jInt(long n) { return mkpair(encodeInt(1), encodeInt(n)); }
D jBool(D b)   { return mkpair(encodeInt(2), b); }
D jStr(const std::string& s) {
    D lst = nilH();
    for (auto it = s.rbegin(); it != s.rend(); ++it) {
        lst = consH(encodeInt((unsigned char)*it), lst);
    }
    return mkpair(encodeInt(3), lst);
}
D jArr(D lst) { return mkpair(encodeInt(4), lst); }
D jObj(D lst) { return mkpair(encodeInt(5), lst); }
D jNull()     { return mkpair(encodeInt(6), D(Fn([](D x) { return x; }))); }

std::string jsonEscape(const std::string& s) {
    std::string out = "\"";
    for (char c : s) {
        if (c == '"') out += "\\\"";
        else if (c == '\\') out += "\\\\";
        else out += c;
    }
    out += "\"";
    return out;
}

std::string decodeJson(D v) {
    long tag = churchToInt(fstp(v));
    D payload = sndp(v);
    switch (tag) {
        case 1:
            return std::to_string(churchToInt(payload));
        case 2:
            return boolToHost(payload) ? "true" : "false";
        case 3: {
            std::string bytes;
            for (D& x : walkL(payload)) bytes += (char)(unsigned char)churchToInt(x);
            return jsonEscape(bytes);
        }
        case 4: {
            std::string out = "[";
            bool first = true;
            for (D& x : walkL(payload)) {
                if (!first) out += ",";
                first = false;
                out += decodeJson(x);
            }
            return out + "]";
        }
        case 5: {
            std::string out = "{";
            bool first = true;
            for (D& pr : walkL(payload)) {
                if (!first) out += ",";
                first = false;
                out += decodeJson(fstp(pr)) + ":" + decodeJson(sndp(pr));
            }
            return out + "}";
        }
        case 6:
            return "null";
    }
    return "?";
}

D _pair = D(Fn([](D _a) { return D(Fn([_a](D _b) { return D(Fn([_a, _b](D _s) { return _s.apply(_a).apply(_b); })); })); }));
D _nil = D(Fn([](D _n) { return D(Fn([_n](D _c) { return _n; })); }));
D _cons = D(Fn([](D _h) { return D(Fn([_h](D _t) { return D(Fn([_h, _t](D _n) { return D(Fn([_h, _t](D _c) { return _c.apply(_h).apply(_t); })); })); })); }));
D _true = D(Fn([](D _t) { return D(Fn([_t](D _f) { return _t; })); }));
D _false = D(Fn([](D _t) { return D(Fn([](D _f) { return _f; })); }));
D _snd = D(Fn([](D _p) { return _p.apply(_false); }));
D _one = D(Fn([](D _f) { return D(Fn([_f](D _x) { return _f.apply(_x); })); }));
D _pred = D(Fn([](D _n) { return D(Fn([_n](D _f) { return D(Fn([_f, _n](D _x) { return _n.apply(D(Fn([_f](D _g) { return D(Fn([_f, _g](D _h) { return _h.apply(_g.apply(_f)); })); }))).apply(D(Fn([_x](D _u) { return _x; }))).apply(D(Fn([](D _u) { return _u; }))); })); })); }));
D _tint = D(Fn([](D _k) { return _pair.apply(_one).apply(_k); }));
D _step = D(Fn([](D _p) { return _p.apply(D(Fn([](D _k) { return D(Fn([_k](D _l) { return _pair.apply(_pred.apply(_k)).apply(_cons.apply(_tint.apply(_k)).apply(_l)); })); }))); }));
D _range = D(Fn([](D _n) { return _snd.apply(_n.apply(_step).apply(_pair.apply(_n).apply(_nil))); }));

int main() {
    check("assert 1", "1", decodeJson(jInt(1)));
    check("assert 2", "true", decodeJson(jBool(_true)));
    check("assert 3", "false", decodeJson(jBool(_false)));
    check("assert 4", "\"hi\"", decodeJson(jStr("hi")));
    check("assert 5", "null", decodeJson(jNull()));
    check("assert 6", "[1,true]", decodeJson(jArr(_cons.apply(jInt(1)).apply(_cons.apply(jBool(_true)).apply(_nil)))));
    check("assert 7", "{\"k\":1}", decodeJson(jObj(_cons.apply(_pair.apply(jStr("k")).apply(jInt(1))).apply(_nil))));
    check("assert 8", "[1,[2,3]]", decodeJson(jArr(_cons.apply(jInt(1)).apply(_cons.apply(jArr(_cons.apply(jInt(2)).apply(_cons.apply(jInt(3)).apply(_nil)))).apply(_nil)))));
    check("assert 9", "[]", decodeJson(jArr(_range.apply(encodeInt(0)))));
    check("assert 10", "[1]", decodeJson(jArr(_range.apply(encodeInt(1)))));
    check("assert 11", "[1,2,3]", decodeJson(jArr(_range.apply(encodeInt(3)))));
    if (_failures > 0) { std::cout << _failures << " failure(s)\n"; return 1; }
    std::cout << "all green\n";
    return 0;
}
