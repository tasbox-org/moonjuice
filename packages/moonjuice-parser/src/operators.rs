use moonjuice_common::Operator;
use moonjuice_common::Operator::*;

pub struct OperatorMetadata {
  pub is_unary: bool,
  pub precedence: u16,
  pub is_right_associative: bool,
}

pub fn get_operator_metadata(operator: Operator) -> OperatorMetadata {
  match operator {
    Assignment => OperatorMetadata {
      is_unary: false,
      precedence: 0,
      is_right_associative: true,
    },

    OptionalCoalesce => OperatorMetadata {
      is_unary: false,
      precedence: 10,
      is_right_associative: true,
    },

    Or => OperatorMetadata {
      is_unary: false,
      precedence: 20,
      is_right_associative: false,
    },
    And => OperatorMetadata {
      is_unary: false,
      precedence: 21,
      is_right_associative: false,
    },

    BitwiseOr => OperatorMetadata {
      is_unary: false,
      precedence: 30,
      is_right_associative: false,
    },
    BitwiseXor => OperatorMetadata {
      is_unary: false,
      precedence: 31,
      is_right_associative: false,
    },
    BitwiseAnd => OperatorMetadata {
      is_unary: false,
      precedence: 32,
      is_right_associative: false,
    },

    Equals | NotEquals => OperatorMetadata {
      is_unary: false,
      precedence: 40,
      is_right_associative: false,
    },

    LessThan | GreaterThan | LessThanOrEqual | GreaterThanOrEqual => OperatorMetadata {
      is_unary: false,
      precedence: 50,
      is_right_associative: false,
    },

    Pipe => OperatorMetadata {
      is_unary: false,
      precedence: 60,
      is_right_associative: false,
    },

    LeftShift | RightShift => OperatorMetadata {
      is_unary: false,
      precedence: 70,
      is_right_associative: false,
    },

    Add | Subtract | Concat => OperatorMetadata {
      is_unary: false,
      precedence: 80,
      is_right_associative: false,
    },
    Multiply | Divide | Modulo => OperatorMetadata {
      is_unary: false,
      precedence: 81,
      is_right_associative: false,
    },

    Not | BitwiseNot | Length => OperatorMetadata {
      is_unary: true,
      precedence: 500,
      is_right_associative: false,
    },

    Index => OperatorMetadata {
      is_unary: false,
      precedence: 1000,
      is_right_associative: false,
    },
  }
}
