#include "src/parser.hpp"

namespace MoonJuice::Parser {
  // Roughly equivalent to the C and Elixir operator precedences
  Parser::OperatorMetadata Parser::getOperatorMetadata(const Operator op) {
    switch (op) {
      case Operator::Assignment:
        return { .precedence = 0, .isRightAssociative = true };

      case Operator::OptionalCoalesce:
        return { .precedence = 10, .isRightAssociative = true };

      case Operator::Or:
        return { .precedence = 20 };
      case Operator::And:
        return { .precedence = 21 };

      case Operator::BitwiseOr:
        return { .precedence = 30 };
      case Operator::BitwiseXor:
        return { .precedence = 31 };
      case Operator::BitwiseAnd:
        return { .precedence = 32 };

      case Operator::Equals:
      case Operator::NotEquals:
        return { .precedence = 40 };

      case Operator::LessThan:
      case Operator::GreaterThan:
      case Operator::LessThanOrEqual:
      case Operator::GreaterThanOrEqual:
        return { .precedence = 50 };

      case Operator::Pipe:
        return { .precedence = 60 };

      case Operator::LeftShift:
      case Operator::RightShift:
        return { .precedence = 70 };

      case Operator::Add:
      case Operator::Subtract:
      case Operator::Concat:
        return { .precedence = 80 };
      case Operator::Multiply:
      case Operator::Divide:
      case Operator::Modulo:
        return { .precedence = 81 };

      case Operator::Not:
      case Operator::BitwiseNot:
      case Operator::Length:
        return { .isUnary = true, .precedence = 500 };

      case Operator::Index:
      case Operator::OptionalIndex:
      case Operator::IndexExpression:
      case Operator::OptionalIndexExpression:
      case Operator::Call:
      case Operator::OptionalCall:
        return { .precedence = 1000 };
    }

    return {};
  }
}
