//! DSL の字句解析。

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Lambda, // λ または \
    Dot,    // .
    Eq,     // =
    EqEq,   // ==
    LParen, // (
    RParen, // )
    LBrack, // [
    RBrack, // ]
    Comma,  // ,
    Sep,    // --- (3個以上のダッシュ)
    Assert, // assert
    Ident(String),
    Int(i64),
    Str(String),
    Eof,
}

pub fn lex(src: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = src.chars().collect();
    let n = chars.len();
    let mut i = 0;
    let mut toks = Vec::new();

    while i < n {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c == '#' {
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        match c {
            'λ' | '\\' => {
                toks.push(Token::Lambda);
                i += 1;
            }
            '.' => {
                toks.push(Token::Dot);
                i += 1;
            }
            '(' => {
                toks.push(Token::LParen);
                i += 1;
            }
            ')' => {
                toks.push(Token::RParen);
                i += 1;
            }
            '[' => {
                toks.push(Token::LBrack);
                i += 1;
            }
            ']' => {
                toks.push(Token::RBrack);
                i += 1;
            }
            ',' => {
                toks.push(Token::Comma);
                i += 1;
            }
            '=' => {
                if i + 1 < n && chars[i + 1] == '=' {
                    toks.push(Token::EqEq);
                    i += 2;
                } else {
                    toks.push(Token::Eq);
                    i += 1;
                }
            }
            '-' => {
                if i + 2 < n && chars[i + 1] == '-' && chars[i + 2] == '-' {
                    while i < n && chars[i] == '-' {
                        i += 1;
                    }
                    toks.push(Token::Sep);
                } else {
                    return Err(format!("unexpected '-' at {}", i));
                }
            }
            '\'' => {
                i += 1;
                let start = i;
                while i < n && chars[i] != '\'' {
                    i += 1;
                }
                if i >= n {
                    return Err("unterminated string literal".to_string());
                }
                let s: String = chars[start..i].iter().collect();
                i += 1; // 閉じ '
                toks.push(Token::Str(s));
            }
            _ if c.is_ascii_digit() => {
                let start = i;
                while i < n && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                let v: i64 = s.parse().map_err(|_| format!("bad int literal: {}", s))?;
                toks.push(Token::Int(v));
            }
            _ if c.is_alphabetic() || c == '_' => {
                let start = i;
                while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                if s == "assert" {
                    toks.push(Token::Assert);
                } else {
                    toks.push(Token::Ident(s));
                }
            }
            _ => return Err(format!("unexpected char {:?} at {}", c, i)),
        }
    }

    toks.push(Token::Eof);
    Ok(toks)
}
