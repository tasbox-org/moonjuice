use moonjuice_common::Operator::*;
use moonjuice_common::SpecialCharacter::{OpenBracket, OpenSquareBracket};
use moonjuice_lexer::Token;
use moonjuice_lexer::TokenValue::{Operator, SpecialCharacter};

pub struct OperatorMetadata {
  pub is_unary: bool,
  pub precedence: u16,
  pub is_right_associative: bool,
}

pub fn get_operator_metadata(token: Token) -> Option<OperatorMetadata> {
  match token.value {
    Operator(Assignment) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 0,
      is_right_associative: true,
    }),

    Operator(OptionalCoalesce) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 10,
      is_right_associative: true,
    }),

    Operator(Or) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 20,
      is_right_associative: false,
    }),
    Operator(And) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 21,
      is_right_associative: false,
    }),

    Operator(BitwiseOr) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 30,
      is_right_associative: false,
    }),
    Operator(BitwiseXor) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 31,
      is_right_associative: false,
    }),
    Operator(BitwiseAnd) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 32,
      is_right_associative: false,
    }),

    Operator(Equals | NotEquals) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 40,
      is_right_associative: false,
    }),

    Operator(LessThan | GreaterThan | LessThanOrEqual | GreaterThanOrEqual) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 50,
      is_right_associative: false,
    }),

    Operator(Pipe) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 60,
      is_right_associative: false,
    }),

    Operator(LeftShift | RightShift) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 70,
      is_right_associative: false,
    }),

    Operator(Add | Subtract | Concat) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 80,
      is_right_associative: false,
    }),
    Operator(Multiply | Divide | Modulo) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 81,
      is_right_associative: false,
    }),

    Operator(Not | BitwiseNot | Length) => Some(OperatorMetadata {
      is_unary: true,
      precedence: 500,
      is_right_associative: false,
    }),

    Operator(Index | OptionalIndex) | SpecialCharacter(OpenBracket | OpenSquareBracket) => Some(OperatorMetadata {
      is_unary: false,
      precedence: 1000,
      is_right_associative: false,
    }),

    _ => None,
  }
}
