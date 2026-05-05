#pragma once

#include "expression.hpp"
#include "shared/language/operator.hpp"

namespace MoonJuice::Parser {
  struct BinaryOperator : NodeBase {
    Operator op = Operator::Add;
    Expression lhs;
    Expression rhs;
  };

  struct UnaryOperator : NodeBase {
    Operator op = Operator::Subtract;
    Expression rhs;
  };

  struct Call : NodeBase {
    Expression lhs;
    std::vector<Expression> arguments;
    bool isOptional = false;
  };
}
