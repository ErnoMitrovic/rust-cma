use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("int")]
    Int,
    #[token("return")]
    Return,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,
    #[regex(r"[0-9]+")]
    Number,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("=")]
    Assign,
    #[token(";")]
    Semicolon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[regex(r"[ \t\n\r]+", logos::skip)]
    Error,
}