use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword,
    Identifier,
    Integer,
    CloseParen,
    OpenParen,
    CloseBrace,
    OpenBrace,
    Semicolon,
    Negation,
    BitwiseComplement,
    LogicalNegation,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub r#type: TokenType,
    pub value: &'a str,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub source: &'a String,
    pub tokens: Vec<Token<'a>>,
}

impl Lexer<'_> {
    pub fn new(src: &String) -> Lexer {
        Lexer {
            source: src,
            tokens: Vec::<Token>::new(),
        }
    }

    pub fn lex(&mut self) -> Result<&Lexer, &'static str> {
        let len = self.source.len();
        let keywords = HashSet::from(["int", "return", "void"]);

        let mut i: usize = 0;

        while i < len.try_into().unwrap() {
            let ch = self.source.as_bytes()[i] as char;

            if ch == '/' {
                i += 1;

                if self.source.as_bytes()[i] as char != '/' {
                    return Err("Error after '/'");
                }

                while self.source.as_bytes()[i] as char != '\n' && i < len.try_into().unwrap() {
                    i += 1;
                }

                i += 1;
            }

            if ch.is_ascii_alphabetic() {
                let mut y: usize = i;

                while (self.source.as_bytes()[y] as char).is_ascii_alphabetic() {
                    y += 1;
                }

                if keywords.contains(&self.source[i..y]) {
                    self.tokens.push(Token {
                        r#type: TokenType::Keyword,
                        value: &self.source[i..y],
                    });
                } else {
                    self.tokens.push(Token {
                        r#type: TokenType::Identifier,
                        value: &self.source[i..y],
                    });
                }

                i = y;

                continue;
            }

            if ch.is_digit(10) {
                let mut y: usize = i;

                while (self.source.as_bytes()[y] as char).is_digit(10) {
                    y += 1;
                }

                self.tokens.push(Token {
                    r#type: TokenType::Integer,
                    value: &self.source[i..y],
                });

                i = y;

                continue;
            }

            if ch == '-' {
                self.tokens.push(Token {
                    r#type: TokenType::Negation,
                    value: "-",
                });

                i += 1;

                continue;
            }

            if ch == '~' {
                self.tokens.push(Token {
                    r#type: TokenType::BitwiseComplement,
                    value: "~",
                });

                i += 1;

                continue;
            }

            if ch == '!' {
                self.tokens.push(Token {
                    r#type: TokenType::LogicalNegation,
                    value: "!",
                });

                i += 1;

                continue;
            }

            if ch == ';' {
                self.tokens.push(Token {
                    r#type: TokenType::Semicolon,
                    value: ";",
                });

                i += 1;

                continue;
            }

            if ch == '{' {
                self.tokens.push(Token {
                    r#type: TokenType::OpenBrace,
                    value: "{",
                });

                i += 1;

                continue;
            }

            if ch == '}' {
                self.tokens.push(Token {
                    r#type: TokenType::CloseBrace,
                    value: "}",
                });

                i += 1;

                continue;
            }

            if ch == '(' {
                self.tokens.push(Token {
                    r#type: TokenType::OpenParen,
                    value: "(",
                });

                i += 1;

                continue;
            }

            if ch == ')' {
                self.tokens.push(Token {
                    r#type: TokenType::CloseParen,
                    value: ")",
                });

                i += 1;

                continue;
            }

            i += 1;
        }

        Ok(self)
    }
}
