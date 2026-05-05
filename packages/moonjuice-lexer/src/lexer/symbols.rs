use crate::error::Error;
use crate::lexer::Lexer;
use crate::token::TokenValue::*;
use crate::token::{Token, TokenValue};
use moonjuice_common::Keyword::*;
use moonjuice_common::Operator::*;
use phf::phf_map;

static SPECIAL_SYMBOLS: phf::Map<&'static str, TokenValue> = phf_map! {
    "break" => Keyword(Break),
  "continue" => Keyword(Continue),
  "return" => Keyword(Return),

  "do" => Keyword(Do),
  "end" => Keyword(End),

  "fn" => Keyword(Function),

  "if" => Keyword(If),
  "then" => Keyword(Then),
  "else" => Keyword(Else),
  "elseif" => Keyword(ElseIf),

  "for" => Keyword(For),
  "in" => Keyword(In),

  "def" => Keyword(Constant),
  "mut" => Keyword(Mutable),
  "export" => Keyword(Export),

  "true" => Bool(true),
  "false" => Bool(false),

  "not" => Operator(Not),
  "and" => Operator(And),
  "or" => Operator(Or),

  "nil" => Nil{},
};

impl Lexer {
  pub(in crate::lexer) fn tokenise_symbol(&mut self) -> Result<Option<Token>, Error> {
    if !self
      .source
      .peek_next()
      .is_some_and(|char| char.is_ascii_alphabetic() || *char == '_')
    {
      return Ok(None);
    }

    while self
      .source
      .peek_next()
      .is_some_and(|char| char.is_ascii_alphanumeric() || *char == '_')
    {
      self.advance()
    }

    let symbol = self.read_lexeme();
    let token_value = SPECIAL_SYMBOLS.get(symbol.as_str()).cloned().unwrap_or(Symbol(symbol));

    Ok(Some(self.new_token(token_value)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use assertor::*;
  use moonjuice_common::Position;
  use parameterized::parameterized;

  #[parameterized(symbol = { "_symbol", "symbol", "Symbol", "symbol1", "sym_bol", "SymBol", "SYM_BOL" })]
  fn should_parse_symbol(symbol: &str) {
    let result = Lexer::tokenise(symbol.chars().collect());

    assert!(result.is_ok());

    let tokens = result.unwrap();
    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Symbol(symbol.to_string()),
      lexeme: symbol.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: symbol.len() + 1,
      },
    }]);
  }
}
