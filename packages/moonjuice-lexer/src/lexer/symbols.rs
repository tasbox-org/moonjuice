use crate::Operator::{And, Not, Or};
use crate::TokenValue::{Bool, Keyword, Nil, Operator, Symbol};
use crate::lexer::Lexer;
use crate::{Token, TokenValue};
use moonjuice_common::Keyword::*;
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

  "nil" => Nil,
};

impl Lexer {
  pub(in crate::lexer) fn tokenise_symbol(&mut self) -> Option<Token> {
    if !self
      .source
      .peek_next()
      .is_some_and(|char| char.is_ascii_alphabetic() || *char == '_')
    {
      return None;
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

    Some(self.new_token(token_value))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Token;
  use crate::TokenValue::Symbol;
  use assertor::*;
  use moonjuice_common::Position;
  use parameterized::parameterized;

  #[parameterized(lexeme = { "_symbol", "symbol", "Symbol", "symbol1", "sym_bol", "SymBol", "SYM_BOL" })]
  fn should_parse_symbol(lexeme: &str) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Symbol(lexeme.to_string()),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }

  #[parameterized(lexeme = { "true", "false" }, value = { true, false })]
  fn should_parse_boolean(lexeme: &str, value: bool) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Bool(value),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }

  #[parameterized(lexeme = { "not", "and", "or" }, value = { Not, And, Or })]
  fn should_parse_operator(lexeme: &str, value: crate::Operator) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Operator(value),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }

  #[test]
  fn should_parse_nil() {
    let tokens = Lexer::tokenise("nil".chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Nil,
      lexeme: "nil".to_string(),
      start: Position { line: 1, column: 1 },
      end: Position { line: 1, column: 4 },
    }]);
  }

  #[parameterized(
    lexeme = {
      "break",
      "continue",
      "return",
      "do",
      "end",
      "fn",
      "if",
      "then",
      "else",
      "elseif",
      "for",
      "in",
      "def",
      "mut",
      "export",
    },
    value = {
      Break,
      Continue,
      Return,
      Do,
      End,
      Function,
      If,
      Then,
      Else,
      ElseIf,
      For,
      In,
      Constant,
      Mutable,
      Export,
    },
  )]
  fn should_parse_keyword(lexeme: &str, value: moonjuice_common::Keyword) {
    let tokens = Lexer::tokenise(lexeme.chars().collect());

    assert_that!(tokens).contains_exactly_in_order(vec![Token {
      value: Keyword(value),
      lexeme: lexeme.to_string(),
      start: Position { line: 1, column: 1 },
      end: Position {
        line: 1,
        column: lexeme.len() + 1,
      },
    }]);
  }
}
