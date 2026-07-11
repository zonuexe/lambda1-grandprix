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

{__CLASS_DECLS__}

var
  _failures: Integer;
{__GLOBALS__}

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

{__CLASS_IMPLS__}

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
{__INIT__}
{__ASSERTS__}
  finish;
end.
