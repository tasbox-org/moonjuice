use crate::nodes::statement::StatementNode;
use moonjuice_common::peekable_stream::PeekableStream;
use moonjuice_lexer::{Lexer, Token};

pub mod nodes;
mod operators;
pub mod parser;
mod tests;

pub struct Parser {
  tokens: PeekableStream<Token>,
}

impl Parser {
  pub fn parse(tokens: Vec<Token>) -> Vec<StatementNode> {
    let mut parser = Self::new(tokens);

    parser.parse_block(|p| p.tokens.has_next())
  }
}

pub fn tokenise_and_parse(source: Vec<char>) -> Vec<StatementNode> {
  let tokens = Lexer::tokenise(source);
  Parser::parse(tokens)
}
