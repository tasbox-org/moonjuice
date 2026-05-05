#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Parser::ParseResult<Expression> Parser::parseUnaryOperator() {
    const auto opToken = peek();
    const auto op = consume<Operator>();
    const auto metadata = getOperatorMetadata(op);

    // Special case for subtract which can be either binary or unary
    if (!metadata.isUnary && op != Operator::Subtract) {
      return error("Expected operand or unary operator", opToken.start, opToken.end);
    }

    auto subExpressionLhs = parseOperand();
    if (std::holds_alternative<Error>(subExpressionLhs)) {
      return std::move(subExpressionLhs);
    }

    auto rhs = parseExpression(std::move(std::get<Expression>(subExpressionLhs)), metadata.precedence);
    if (std::holds_alternative<Error>(rhs)) {
      return std::move(rhs);
    }

    auto node = UnaryOperator{
      .op = op,
      .rhs = std::move(std::get<Expression>(rhs)),
    };
    node.start = opToken.start;
    node.end = opToken.end;

    return std::make_shared<ExpressionValue>(std::move(node));
  }
}
