//! DSL の再帰下降パーサ。
//!
//! 文法（要約）:
//!   program = def* '---' assert*
//!   def     = ident '=' expr
//!   assert  = 'assert' expr '==' expr
//!   expr    = lam | app
//!   lam     = LAMBDA ident+ '.' expr        （多引数は入れ子へ脱糖）
//!   app     = atom atom*                     （並置・左結合）
//!   atom    = '(' expr ')' | ident ('[' args ']')? | int | str
//!   args    = expr (',' expr)*

use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    toks: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(toks: Vec<Token>) -> Self {
        Parser { toks, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.toks[self.pos]
    }

    fn bump(&mut self) -> Token {
        let t = self.toks[self.pos].clone();
        self.pos += 1;
        t
    }

    fn eat(&mut self, t: &Token) -> Result<(), String> {
        if self.peek() == t {
            self.pos += 1;
            Ok(())
        } else {
            Err(format!("expected {:?}, got {:?}", t, self.peek()))
        }
    }

    fn ident(&mut self) -> Result<String, String> {
        match self.bump() {
            Token::Ident(s) => Ok(s),
            other => Err(format!("expected ident, got {:?}", other)),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut defs = Vec::new();
        while !matches!(self.peek(), Token::Sep | Token::Eof) {
            defs.push(self.parse_def()?);
        }
        if matches!(self.peek(), Token::Sep) {
            self.bump();
        }
        let mut asserts = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            asserts.push(self.parse_assert()?);
        }
        Ok(Program { defs, asserts })
    }

    fn parse_def(&mut self) -> Result<Def, String> {
        let name = self.ident()?;
        self.eat(&Token::Eq)?;
        let term = self.parse_expr()?;
        Ok(Def { name, term })
    }

    fn parse_assert(&mut self) -> Result<Assert, String> {
        self.eat(&Token::Assert)?;
        let lhs = self.parse_expr()?;
        self.eat(&Token::EqEq)?;
        let rhs = self.parse_expr()?;
        Ok(Assert { lhs, rhs })
    }

    fn parse_expr(&mut self) -> Result<Term, String> {
        if matches!(self.peek(), Token::Lambda) {
            self.parse_lam()
        } else {
            self.parse_app()
        }
    }

    fn parse_lam(&mut self) -> Result<Term, String> {
        self.eat(&Token::Lambda)?;
        let mut params = Vec::new();
        while let Token::Ident(_) = self.peek() {
            params.push(self.ident()?);
        }
        if params.is_empty() {
            return Err("lambda needs at least one parameter".to_string());
        }
        self.eat(&Token::Dot)?;
        let body = self.parse_expr()?;
        // 多引数を入れ子の単引数 λ へ脱糖
        let mut term = body;
        for p in params.into_iter().rev() {
            term = Term::Lam(p, Box::new(term));
        }
        Ok(term)
    }

    fn starts_atom(&self) -> bool {
        matches!(
            self.peek(),
            Token::Ident(_) | Token::Int(_) | Token::Str(_) | Token::LParen
        )
    }

    /// 次が `ident =`（次の定義の開始）か。式中に単独の `=` は現れないので、
    /// これを適用連鎖の終端シグナルに使える（定義は改行で区切られないため）。
    fn at_def_start(&self) -> bool {
        matches!(self.peek(), Token::Ident(_)) && matches!(self.toks.get(self.pos + 1), Some(Token::Eq))
    }

    fn parse_app(&mut self) -> Result<Term, String> {
        let mut t = self.parse_atom()?;
        while self.starts_atom() && !self.at_def_start() {
            let arg = self.parse_atom()?;
            t = Term::App(Box::new(t), Box::new(arg));
        }
        Ok(t)
    }

    fn parse_atom(&mut self) -> Result<Term, String> {
        match self.peek().clone() {
            Token::LParen => {
                self.bump();
                let e = self.parse_expr()?;
                self.eat(&Token::RParen)?;
                Ok(e)
            }
            Token::Ident(name) => {
                self.bump();
                if matches!(self.peek(), Token::LBrack) {
                    self.bump();
                    let mut args = Vec::new();
                    if !matches!(self.peek(), Token::RBrack) {
                        loop {
                            args.push(self.parse_expr()?);
                            if matches!(self.peek(), Token::Comma) {
                                self.bump();
                            } else {
                                break;
                            }
                        }
                    }
                    self.eat(&Token::RBrack)?;
                    Ok(Term::HostCall(name, args))
                } else {
                    Ok(Term::Var(name))
                }
            }
            Token::Int(v) => {
                self.bump();
                Ok(Term::IntLit(v))
            }
            Token::Str(s) => {
                self.bump();
                Ok(Term::StrLit(s))
            }
            other => Err(format!("unexpected token in atom: {:?}", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    fn parse(src: &str) -> Program {
        let toks = lex(src).expect("lex");
        Parser::new(toks).parse_program().expect("parse")
    }

    #[test]
    fn app_is_left_assoc() {
        let p = parse("f = a b c\n---\n");
        // ((a b) c)
        match &p.defs[0].term {
            Term::App(l, r) => {
                assert!(matches!(**r, Term::Var(ref s) if s == "c"));
                assert!(matches!(**l, Term::App(_, _)));
            }
            _ => panic!("expected app"),
        }
    }

    #[test]
    fn multi_param_lambda_desugars() {
        let p = parse("k = λx y.x\n---\n");
        match &p.defs[0].term {
            Term::Lam(_, inner) => assert!(matches!(**inner, Term::Lam(_, _))),
            _ => panic!("expected nested lam"),
        }
    }

    #[test]
    fn host_call_vs_application() {
        let p = parse("---\nassert '1' == decodeInt[ I (encodeInt[1]) ]\n");
        let a = &p.asserts[0];
        assert!(matches!(a.lhs, Term::StrLit(ref s) if s == "1"));
        match &a.rhs {
            Term::HostCall(name, args) => {
                assert_eq!(name, "decodeInt");
                assert!(matches!(args[0], Term::App(_, _)));
            }
            _ => panic!("expected host call"),
        }
    }
}
