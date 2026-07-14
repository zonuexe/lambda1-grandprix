program Main;
{$mode objfpc}{$H+}
uses SysUtils;

// λ-1 translator — Free Pascal prelude（クロージャ変換）。
// FPC 3.2.2 は無名関数を持たないため、各 λ を「捕捉した自由変数フィールド＋
// Apply メソッド」を持つクラスへ変換する（生成部が {__CLASS_*__} に入る）。
// 万能型 TValue はタグ付き union（TNum/TStr ＋ 関数＝クロージャオブジェクト）。

type
  TValue = class
    function Apply(x: TValue): TValue; virtual;
  end;

  TNum = class(TValue)
  public
    n: Integer;
    constructor Create(a: Integer);
  end;

  TStr = class(TValue)
  public
    s: string;
    constructor Create(a: string);
  end;

  // チャーチ数（λf.λx. f^n x）を作る固定クロージャ。
  TChurchF = class(TValue)
  public
    cnt: Integer;
    constructor Create(a: Integer);
    function Apply(arg: TValue): TValue; override;
  end;

  TChurchFX = class(TValue)
  public
    cnt: Integer;
    fn: TValue;
    constructor Create(a: Integer; b: TValue);
    function Apply(arg: TValue): TValue; override;
  end;

  TIncr = class(TValue)
    function Apply(arg: TValue): TValue; override;
  end;

  // --- JSON 層（ADR-0005）で使う固定クロージャ群 ---
  TValueArr = array of TValue;

  TIdent = class(TValue)              // λx.x
    function Apply(arg: TValue): TValue; override;
  end;
  TSelKa = class(TValue)              // (λa.λb.a) を a に適用した後 = λb.a
  public
    fa: TValue;
    constructor CreateC(a: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TSelK = class(TValue)               // λa.λb.a （K = nil = cTrue 兼用）
    function Apply(arg: TValue): TValue; override;
  end;
  TSelKI = class(TValue)              // λa.λb.b （KI = cFalse 兼用）
    function Apply(arg: TValue): TValue; override;
  end;
  TPair2 = class(TValue)              // λs. s a b
  public
    fa, fb: TValue;
    constructor CreateC(a, b: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TConsB = class(TValue)              // λc. c h t
  public
    fh, ft: TValue;
    constructor CreateC(h, t: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TConsA = class(TValue)              // λn.λc. c h t （cons h t）
  public
    fh, ft: TValue;
    constructor CreateC(h, t: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TFalseCaseB = class(TValue)         // λt. cFalse
    function Apply(arg: TValue): TValue; override;
  end;
  TFalseCaseA = class(TValue)         // λh.λt. cFalse
    function Apply(arg: TValue): TValue; override;
  end;

  TClo2 = class(TValue)
  public
    f_a: TValue;
    f_b: TValue;
    constructor CreateC(p_a: TValue; p_b: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo1 = class(TValue)
  public
    f_a: TValue;
    constructor CreateC(p_a: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo0 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo4 = class(TValue)
  public
    f_n: TValue;
    constructor CreateC(p_n: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo3 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo8 = class(TValue)
  public
    f_h: TValue;
    f_t: TValue;
    constructor CreateC(p_h: TValue; p_t: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo7 = class(TValue)
  public
    f_h: TValue;
    f_t: TValue;
    constructor CreateC(p_h: TValue; p_t: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo6 = class(TValue)
  public
    f_h: TValue;
    constructor CreateC(p_h: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo5 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo10 = class(TValue)
  public
    f_t: TValue;
    constructor CreateC(p_t: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo9 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo12 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo11 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo13 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo15 = class(TValue)
  public
    f_f: TValue;
    constructor CreateC(p_f: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo14 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo20 = class(TValue)
  public
    f_f: TValue;
    f_g: TValue;
    constructor CreateC(p_f: TValue; p_g: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo19 = class(TValue)
  public
    f_f: TValue;
    constructor CreateC(p_f: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo21 = class(TValue)
  public
    f_x: TValue;
    constructor CreateC(p_x: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo22 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo18 = class(TValue)
  public
    f_f: TValue;
    f_n: TValue;
    constructor CreateC(p_f: TValue; p_n: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo17 = class(TValue)
  public
    f_n: TValue;
    constructor CreateC(p_n: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo16 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo23 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo26 = class(TValue)
  public
    f_k: TValue;
    constructor CreateC(p_k: TValue);
    function Apply(arg: TValue): TValue; override;
  end;
  TClo25 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo24 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;
  TClo27 = class(TValue)
  public
    function Apply(arg: TValue): TValue; override;
  end;


var
  _failures: Integer;
  _pair: TValue;
  _nil: TValue;
  _cons: TValue;
  _true: TValue;
  _false: TValue;
  _snd: TValue;
  _one: TValue;
  _pred: TValue;
  _tint: TValue;
  _step: TValue;
  _range: TValue;


function TValue.Apply(x: TValue): TValue;
begin
  raise Exception.Create('applied a non-function');
  Result := nil;
end;

constructor TNum.Create(a: Integer);
begin
  n := a;
end;

constructor TStr.Create(a: string);
begin
  s := a;
end;

constructor TChurchF.Create(a: Integer);
begin
  cnt := a;
end;

function TChurchF.Apply(arg: TValue): TValue;
begin
  Result := TChurchFX.Create(cnt, arg);
end;

constructor TChurchFX.Create(a: Integer; b: TValue);
begin
  cnt := a;
  fn := b;
end;

function TChurchFX.Apply(arg: TValue): TValue;
var
  i: Integer;
  r: TValue;
begin
  r := arg;
  for i := 1 to cnt do
    r := fn.Apply(r);
  Result := r;
end;

function TIncr.Apply(arg: TValue): TValue;
begin
  Result := TNum.Create(TNum(arg).n + 1);
end;

// --- JSON 層クロージャの実装 ---
function TIdent.Apply(arg: TValue): TValue;
begin
  Result := arg;
end;

constructor TSelKa.CreateC(a: TValue);
begin
  fa := a;
end;
function TSelKa.Apply(arg: TValue): TValue;
begin
  Result := fa;
end;

function TSelK.Apply(arg: TValue): TValue;
begin
  Result := TSelKa.CreateC(arg);
end;

function TSelKI.Apply(arg: TValue): TValue;
begin
  Result := TIdent.Create;
end;

constructor TPair2.CreateC(a, b: TValue);
begin
  fa := a;
  fb := b;
end;
function TPair2.Apply(arg: TValue): TValue;
begin
  Result := arg.Apply(fa).Apply(fb);
end;

constructor TConsB.CreateC(h, t: TValue);
begin
  fh := h;
  ft := t;
end;
function TConsB.Apply(arg: TValue): TValue;
begin
  Result := arg.Apply(fh).Apply(ft);
end;

constructor TConsA.CreateC(h, t: TValue);
begin
  fh := h;
  ft := t;
end;
function TConsA.Apply(arg: TValue): TValue;
begin
  Result := TConsB.CreateC(fh, ft);
end;

function TFalseCaseB.Apply(arg: TValue): TValue;
begin
  Result := TSelKI.Create;
end;
function TFalseCaseA.Apply(arg: TValue): TValue;
begin
  Result := TFalseCaseB.Create;
end;

constructor TClo2.CreateC(p_a: TValue; p_b: TValue);
begin
  f_a := p_a;
  f_b := p_b;
end;

function TClo2.Apply(arg: TValue): TValue;
begin
  Result := arg.Apply(f_a).Apply(f_b);
end;

constructor TClo1.CreateC(p_a: TValue);
begin
  f_a := p_a;
end;

function TClo1.Apply(arg: TValue): TValue;
begin
  Result := TClo2.CreateC(f_a, arg);
end;

function TClo0.Apply(arg: TValue): TValue;
begin
  Result := TClo1.CreateC(arg);
end;

constructor TClo4.CreateC(p_n: TValue);
begin
  f_n := p_n;
end;

function TClo4.Apply(arg: TValue): TValue;
begin
  Result := f_n;
end;

function TClo3.Apply(arg: TValue): TValue;
begin
  Result := TClo4.CreateC(arg);
end;

constructor TClo8.CreateC(p_h: TValue; p_t: TValue);
begin
  f_h := p_h;
  f_t := p_t;
end;

function TClo8.Apply(arg: TValue): TValue;
begin
  Result := arg.Apply(f_h).Apply(f_t);
end;

constructor TClo7.CreateC(p_h: TValue; p_t: TValue);
begin
  f_h := p_h;
  f_t := p_t;
end;

function TClo7.Apply(arg: TValue): TValue;
begin
  Result := TClo8.CreateC(f_h, f_t);
end;

constructor TClo6.CreateC(p_h: TValue);
begin
  f_h := p_h;
end;

function TClo6.Apply(arg: TValue): TValue;
begin
  Result := TClo7.CreateC(f_h, arg);
end;

function TClo5.Apply(arg: TValue): TValue;
begin
  Result := TClo6.CreateC(arg);
end;

constructor TClo10.CreateC(p_t: TValue);
begin
  f_t := p_t;
end;

function TClo10.Apply(arg: TValue): TValue;
begin
  Result := f_t;
end;

function TClo9.Apply(arg: TValue): TValue;
begin
  Result := TClo10.CreateC(arg);
end;

function TClo12.Apply(arg: TValue): TValue;
begin
  Result := arg;
end;

function TClo11.Apply(arg: TValue): TValue;
begin
  Result := TClo12.Create;
end;

function TClo13.Apply(arg: TValue): TValue;
begin
  Result := arg.Apply(_false);
end;

constructor TClo15.CreateC(p_f: TValue);
begin
  f_f := p_f;
end;

function TClo15.Apply(arg: TValue): TValue;
begin
  Result := f_f.Apply(arg);
end;

function TClo14.Apply(arg: TValue): TValue;
begin
  Result := TClo15.CreateC(arg);
end;

constructor TClo20.CreateC(p_f: TValue; p_g: TValue);
begin
  f_f := p_f;
  f_g := p_g;
end;

function TClo20.Apply(arg: TValue): TValue;
begin
  Result := arg.Apply(f_g.Apply(f_f));
end;

constructor TClo19.CreateC(p_f: TValue);
begin
  f_f := p_f;
end;

function TClo19.Apply(arg: TValue): TValue;
begin
  Result := TClo20.CreateC(f_f, arg);
end;

constructor TClo21.CreateC(p_x: TValue);
begin
  f_x := p_x;
end;

function TClo21.Apply(arg: TValue): TValue;
begin
  Result := f_x;
end;

function TClo22.Apply(arg: TValue): TValue;
begin
  Result := arg;
end;

constructor TClo18.CreateC(p_f: TValue; p_n: TValue);
begin
  f_f := p_f;
  f_n := p_n;
end;

function TClo18.Apply(arg: TValue): TValue;
begin
  Result := f_n.Apply(TClo19.CreateC(f_f)).Apply(TClo21.CreateC(arg)).Apply(TClo22.Create);
end;

constructor TClo17.CreateC(p_n: TValue);
begin
  f_n := p_n;
end;

function TClo17.Apply(arg: TValue): TValue;
begin
  Result := TClo18.CreateC(arg, f_n);
end;

function TClo16.Apply(arg: TValue): TValue;
begin
  Result := TClo17.CreateC(arg);
end;

function TClo23.Apply(arg: TValue): TValue;
begin
  Result := _pair.Apply(_one).Apply(arg);
end;

constructor TClo26.CreateC(p_k: TValue);
begin
  f_k := p_k;
end;

function TClo26.Apply(arg: TValue): TValue;
begin
  Result := _pair.Apply(_pred.Apply(f_k)).Apply(_cons.Apply(_tint.Apply(f_k)).Apply(arg));
end;

function TClo25.Apply(arg: TValue): TValue;
begin
  Result := TClo26.CreateC(arg);
end;

function TClo24.Apply(arg: TValue): TValue;
begin
  Result := arg.Apply(TClo25.Create);
end;

function TClo27.Apply(arg: TValue): TValue;
begin
  Result := _snd.Apply(arg.Apply(_step).Apply(_pair.Apply(arg).Apply(_nil)));
end;



function encodeInt(nn: Integer): TValue;
begin
  Result := TChurchF.Create(nn);
end;

function decodeInt(v: TValue): string;
var
  r: TValue;
begin
  r := v.Apply(TIncr.Create).Apply(TNum.Create(0));
  Result := IntToStr(TNum(r).n);
end;

function decodeBool(v: TValue): string;
var
  r: TValue;
begin
  r := v.Apply(TStr.Create('true')).Apply(TStr.Create('false'));
  Result := TStr(r).s;
end;

// ============ JSON 層（型付き値。ADR-0005） ============
function mkpair(a, b: TValue): TValue;
begin
  Result := TPair2.CreateC(a, b);
end;

function fstp(p: TValue): TValue;
begin
  Result := p.Apply(TSelK.Create);
end;

function sndp(p: TValue): TValue;
begin
  Result := p.Apply(TSelKI.Create);
end;

function churchToInt(c: TValue): Integer;
var
  r: TValue;
begin
  r := c.Apply(TIncr.Create).Apply(TNum.Create(0));
  Result := TNum(r).n;
end;

function boolToHost(c: TValue): Boolean;
var
  r: TValue;
begin
  r := c.Apply(TStr.Create('T')).Apply(TStr.Create('F'));
  Result := TStr(r).s = 'T';
end;

function consH(h, t: TValue): TValue;
begin
  Result := TConsA.CreateC(h, t);
end;

function isNil(lst: TValue): Boolean;
var
  r: TValue;
begin
  r := lst.Apply(TSelK.Create).Apply(TFalseCaseA.Create);
  Result := boolToHost(r);
end;

function headL(lst: TValue): TValue;
begin
  Result := lst.Apply(TStr.Create('')).Apply(TSelK.Create);
end;

function tailL(lst: TValue): TValue;
begin
  Result := lst.Apply(TStr.Create('')).Apply(TSelKI.Create);
end;

function walk(lst: TValue): TValueArr;
var
  outv: TValueArr;
begin
  SetLength(outv, 0);
  while not isNil(lst) do
  begin
    SetLength(outv, Length(outv) + 1);
    outv[High(outv)] := headL(lst);
    lst := tailL(lst);
  end;
  Result := outv;
end;

function jInt(n: Integer): TValue;
begin
  Result := mkpair(encodeInt(1), encodeInt(n));
end;

function jBool(b: TValue): TValue;
begin
  Result := mkpair(encodeInt(2), b);
end;

function jStr(s: string): TValue;
var
  lst: TValue;
  i: Integer;
begin
  lst := TSelK.Create;  // nil
  for i := Length(s) downto 1 do
    lst := consH(encodeInt(Ord(s[i])), lst);
  Result := mkpair(encodeInt(3), lst);
end;

function jArr(lst: TValue): TValue;
begin
  Result := mkpair(encodeInt(4), lst);
end;

function jObj(lst: TValue): TValue;
begin
  Result := mkpair(encodeInt(5), lst);
end;

function jNull: TValue;
begin
  Result := mkpair(encodeInt(6), TIdent.Create);
end;

function jsonEscape(s: string): string;
var
  i: Integer;
  c: Char;
  outp: string;
begin
  outp := '"';
  for i := 1 to Length(s) do
  begin
    c := s[i];
    if c = '"' then
      outp := outp + '\"'
    else if c = '\' then
      outp := outp + '\\'
    else
      outp := outp + c;
  end;
  Result := outp + '"';
end;

function decodeJson(v: TValue): string;
var
  tag, i: Integer;
  payload, pr: TValue;
  xs: TValueArr;
  bytes, parts: string;
begin
  tag := churchToInt(fstp(v));
  payload := sndp(v);
  case tag of
    1: Result := IntToStr(churchToInt(payload));
    2: if boolToHost(payload) then Result := 'true' else Result := 'false';
    3: begin
         xs := walk(payload);
         bytes := '';
         for i := 0 to High(xs) do
           bytes := bytes + Chr(churchToInt(xs[i]));
         Result := jsonEscape(bytes);
       end;
    4: begin
         xs := walk(payload);
         parts := '';
         for i := 0 to High(xs) do
         begin
           if i > 0 then parts := parts + ',';
           parts := parts + decodeJson(xs[i]);
         end;
         Result := '[' + parts + ']';
       end;
    5: begin
         xs := walk(payload);
         parts := '';
         for i := 0 to High(xs) do
         begin
           if i > 0 then parts := parts + ',';
           pr := xs[i];
           parts := parts + decodeJson(fstp(pr)) + ':' + decodeJson(sndp(pr));
         end;
         Result := '{' + parts + '}';
       end;
    6: Result := 'null';
  else
    Result := '?';
  end;
end;

procedure check(const label_: string; const a: string; const b: string);
begin
  if a = b then
    WriteLn('ok   ', label_)
  else
  begin
    Inc(_failures);
    WriteLn('FAIL ', label_, ': ', a, ' != ', b);
  end;
end;

procedure finish;
begin
  if _failures > 0 then
  begin
    WriteLn(_failures, ' failure(s)');
    Halt(1);
  end;
  WriteLn('all green');
end;

begin
  _failures := 0;
  _pair := TClo0.Create;
  _nil := TClo3.Create;
  _cons := TClo5.Create;
  _true := TClo9.Create;
  _false := TClo11.Create;
  _snd := TClo13.Create;
  _one := TClo14.Create;
  _pred := TClo16.Create;
  _tint := TClo23.Create;
  _step := TClo24.Create;
  _range := TClo27.Create;

  check('assert 1', '1', decodeJson(jInt(1)));
  check('assert 2', 'true', decodeJson(jBool(_true)));
  check('assert 3', 'false', decodeJson(jBool(_false)));
  check('assert 4', '"hi"', decodeJson(jStr('hi')));
  check('assert 5', 'null', decodeJson(jNull()));
  check('assert 6', '[1,true]', decodeJson(jArr(_cons.Apply(jInt(1)).Apply(_cons.Apply(jBool(_true)).Apply(_nil)))));
  check('assert 7', '{"k":1}', decodeJson(jObj(_cons.Apply(_pair.Apply(jStr('k')).Apply(jInt(1))).Apply(_nil))));
  check('assert 8', '[1,[2,3]]', decodeJson(jArr(_cons.Apply(jInt(1)).Apply(_cons.Apply(jArr(_cons.Apply(jInt(2)).Apply(_cons.Apply(jInt(3)).Apply(_nil)))).Apply(_nil)))));
  check('assert 9', '[]', decodeJson(jArr(_range.Apply(encodeInt(0)))));
  check('assert 10', '[1]', decodeJson(jArr(_range.Apply(encodeInt(1)))));
  check('assert 11', '[1,2,3]', decodeJson(jArr(_range.Apply(encodeInt(3)))));

  finish;
end.
