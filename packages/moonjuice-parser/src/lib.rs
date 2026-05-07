use crate::nodes::expression::ExpressionNode;
use moonjuice_common::peekable_stream::PeekableStream;
use moonjuice_lexer::{Lexer, Token};

pub mod nodes;
mod operators;
pub mod parser;

pub struct Parser {
  tokens: PeekableStream<Token>,
}

impl Parser {
  pub fn parse(tokens: Vec<Token>) -> ExpressionNode {
    let mut parser = Self::new(tokens);

    parser.parse_block(|p| p.tokens.has_next())
  }
}

pub fn tokenise_and_parse(source: Vec<char>) -> ExpressionNode {
  let tokens = Lexer::tokenise(source);
  Parser::parse(tokens)
}
