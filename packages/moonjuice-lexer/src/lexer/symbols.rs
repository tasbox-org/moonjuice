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
  use rstest::rstest;

  #[rstest]
  #[case("_symbol")]
  #[case("symbol")]
  #[case("Symbol")]
  #[case("symbol1")]
  #[case("sym_bol")]
  #[case("SymBol")]
  #[case("SYM_BOL")]
  fn should_parse_symbol(#[case] lexeme: &str) {
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

  #[rstest]
  #[case("true", true)]
  #[case("false", false)]
  fn should_parse_boolean(#[case] lexeme: &str, #[case] value: bool) {
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

  #[rstest]
  #[case("not", Not)]
  #[case("and", And)]
  #[case("or", Or)]
  fn should_parse_operator(#[case] lexeme: &str, #[case] value: crate::Operator) {
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

  #[rstest]
  #[case("break", Break)]
  #[case("continue", Continue)]
  #[case("return", Return)]
  #[case("do", Do)]
  #[case("end", End)]
  #[case("fn", Function)]
  #[case("if", If)]
  #[case("then", Then)]
  #[case("else", Else)]
  #[case("elseif", ElseIf)]
  #[case("for", For)]
  #[case("in", In)]
  #[case("def", Constant)]
  #[case("mut", Mutable)]
  #[case("export", Export)]
  fn should_parse_keyword(#[case] lexeme: &str, #[case] value: moonjuice_common::Keyword) {
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
