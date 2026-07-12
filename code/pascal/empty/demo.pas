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



var
  _failures: Integer;


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


  finish;
end.
