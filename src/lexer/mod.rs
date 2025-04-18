pub mod token;

use logos::Logos;
use token::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    Token::lexer(input).collect()
}