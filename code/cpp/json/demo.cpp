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

D pair = D(Fn([](D a) { return D(Fn([a](D b) { return D(Fn([a, b](D s) { return s.apply(a).apply(b); })); })); }));
D nil = D(Fn([](D n) { return D(Fn([n](D c) { return n; })); }));
D cons = D(Fn([](D h) { return D(Fn([h](D t) { return D(Fn([h, t](D n) { return D(Fn([h, t](D c) { return c.apply(h).apply(t); })); })); })); }));
D _true = D(Fn([](D t) { return D(Fn([t](D f) { return t; })); }));
D _false = D(Fn([](D t) { return D(Fn([](D f) { return f; })); }));
D snd = D(Fn([](D p) { return p.apply(_false); }));
D one = D(Fn([](D f) { return D(Fn([f](D x) { return f.apply(x); })); }));
D pred = D(Fn([](D n) { return D(Fn([n](D f) { return D(Fn([f, n](D x) { return n.apply(D(Fn([f](D g) { return D(Fn([f, g](D h) { return h.apply(g.apply(f)); })); }))).apply(D(Fn([x](D u) { return x; }))).apply(D(Fn([](D u) { return u; }))); })); })); }));
D tint = D(Fn([](D k) { return pair.apply(one).apply(k); }));
D step = D(Fn([](D p) { return p.apply(D(Fn([](D k) { return D(Fn([k](D l) { return pair.apply(pred.apply(k)).apply(cons.apply(tint.apply(k)).apply(l)); })); }))); }));
D range = D(Fn([](D n) { return snd.apply(n.apply(step).apply(pair.apply(n).apply(nil))); }));

int main() {
    check("assert 1", "1", decodeJson(jInt(1)));
    check("assert 2", "true", decodeJson(jBool(_true)));
    check("assert 3", "false", decodeJson(jBool(_false)));
    check("assert 4", "\"hi\"", decodeJson(jStr("hi")));
    check("assert 5", "null", decodeJson(jNull()));
    check("assert 6", "[1,true]", decodeJson(jArr(cons.apply(jInt(1)).apply(cons.apply(jBool(_true)).apply(nil)))));
    check("assert 7", "{\"k\":1}", decodeJson(jObj(cons.apply(pair.apply(jStr("k")).apply(jInt(1))).apply(nil))));
    check("assert 8", "[1,[2,3]]", decodeJson(jArr(cons.apply(jInt(1)).apply(cons.apply(jArr(cons.apply(jInt(2)).apply(cons.apply(jInt(3)).apply(nil)))).apply(nil)))));
    check("assert 9", "[]", decodeJson(jArr(range.apply(encodeInt(0)))));
    check("assert 10", "[1]", decodeJson(jArr(range.apply(encodeInt(1)))));
    check("assert 11", "[1,2,3]", decodeJson(jArr(range.apply(encodeInt(3)))));
    if (_failures > 0) { std::cout << _failures << " failure(s)\n"; return 1; }
    std::cout << "all green\n";
    return 0;
}
