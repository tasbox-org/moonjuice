#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Parser::ParseResult<Expression> Parser::parseExpression() {
    auto lhs = parseOperand();
    if (std::holds_alternative<Error>(lhs)) {
      return std::get<Error>(lhs);
    }

    return parseExpression(std::move(std::get<Expression>(lhs)), 0);
  }

  Parser::ParseResult<Expression> Parser::parseExpression(Expression lhs, Precedence minPrecedence) {
    auto lookahead = peek();

    while (isBinaryOperatorWithGreaterOrEqualPrecedence(lookahead, minPrecedence)) {
      ParseResult<Expression> result;

      if (nextIs(SpecialCharacter::OpenBracket) || nextIs(Operator::OptionalCall)) {
        result = parseCall(std::move(lhs));
      } else if (nextIs(SpecialCharacter::OpenSquareBracket) || nextIs(Operator::OptionalIndexExpression)) {
        result = parseIndexExpression(std::move(lhs));
      } else if (nextIs<Operator>()) {
        result = parseBinaryOperator(std::move(lhs));
      }

      if (std::holds_alternative<Error>(result)) {
        return std::get<Error>(result);
      }

      lhs = std::move(std::get<Expression>(result));
      lookahead = peek();
    }

    return std::move(lhs);
  }

  Parser::ParseResult<Expression> Parser::parseBinaryOperator(Expression lhs) {
    const auto op = consume<Operator>();
    const auto opPrecedence = getOperatorMetadata(op).precedence;
    const auto start = lookBack().start;
    const auto end = lookBack().end;

    auto rhs = parseOperand();
    if (std::holds_alternative<Error>(rhs)) {
      return std::get<Error>(rhs);
    }

    auto lookahead = peek();
    while (isRightHandSideChainedOperator(lookahead, opPrecedence)) {
      const auto rightHandOp = getIfSubExpressionBinaryOperator(lookahead).value();
      const auto lookaheadPrecedence = getOperatorMetadata(rightHandOp).precedence;

      rhs = parseExpression(
        std::move(std::get<Expression>(rhs)),
        lookaheadPrecedence > opPrecedence ? opPrecedence + 1 : opPrecedence
      );
      if (std::holds_alternative<Error>(rhs)) {
        return std::get<Error>(rhs);
      }

      lookahead = peek();
    }

    const auto isAssignment = op == Operator::Assignment;
    const auto isSymbol = std::holds_alternative<Symbol>(*lhs);
    const auto isIndexOperation = std::holds_alternative<BinaryOperator>(*lhs) && (
      std::get<BinaryOperator>(*lhs).op == Operator::Index
      || std::get<BinaryOperator>(*lhs).op == Operator::IndexExpression
    );

    if (isAssignment && !isSymbol && !isIndexOperation) {
      return errorNext(
        "Expected symbol or index expression (without optional chaining) on left side of assignment operator"
      );
    }

    auto node = BinaryOperator{
      .op = op,
      .lhs = std::move(lhs),
      .rhs = std::move(std::get<Expression>(rhs)),
    };
    node.start = start;
    node.end = end;

    return std::make_shared<ExpressionValue>(std::move(node));
  }

  Parser::ParseResult<Expression> Parser::parseCall(Expression lhs) {
    const auto isOptional = nextIs(Operator::OptionalCall);
    consume();

    auto node = Call{
      .lhs = std::move(lhs),
      .isOptional = isOptional,
    };
    node.start = lookBack().start;

    if (!nextIs(SpecialCharacter::CloseBracket)) {
      do {
        auto argument = parseExpression();
        if (std::holds_alternative<Error>(argument)) {
          return std::move(argument);
        }

        node.arguments.push_back(std::move(std::get<Expression>(argument)));
      } while (consumeIf(SpecialCharacter::Comma));
    }

    if (!consumeIf(SpecialCharacter::CloseBracket)) {
      return errorNext("Expected ')' to close function argument list");
    }

    node.end = lookBack().end;
    return std::make_shared<ExpressionValue>(std::move(node));
  }

  Parser::ParseResult<Expression> Parser::parseIndexExpression(Expression lhs) {
    const auto isOptional = nextIs(Operator::OptionalIndexExpression);
    consume();

    auto node = BinaryOperator{
      .op = isOptional ? Operator::OptionalIndexExpression : Operator::IndexExpression,
      .lhs = std::move(lhs),
    };
    node.start = lookBack().start;

    auto key = parseExpression();
    if (std::holds_alternative<Error>(key)) {
      return std::move(key);
    }

    if (!consumeIf(SpecialCharacter::CloseSquareBracket)) {
      return errorNext("Expected ']' to close table index");
    }

    node.rhs = std::move(std::get<Expression>(key));

    node.end = lookBack().end;
    return std::make_shared<ExpressionValue>(std::move(node));
  }

  bool Parser::isBinaryOperatorWithGreaterOrEqualPrecedence(const Lexer::Token& token, const Precedence minPrecedence) {
    const auto op = getIfSubExpressionBinaryOperator(token);
    if (!op.has_value()) {
      return false;
    }

    const auto metadata = getOperatorMetadata(op.value());
    return metadata.precedence >= minPrecedence;
  }

  bool Parser::isRightHandSideChainedOperator(const Lexer::Token& token, const Precedence previousOperatorPrecedence) {
    const auto op = getIfSubExpressionBinaryOperator(token);
    if (!op.has_value()) {
      return false;
    }

    const auto metadata = getOperatorMetadata(op.value());
    return metadata.isRightAssociative
      ? metadata.precedence >= previousOperatorPrecedence
      : metadata.precedence > previousOperatorPrecedence;
  }

  std::optional<Operator> Parser::getIfSubExpressionBinaryOperator(const Lexer::Token& token) {
    if (std::holds_alternative<Operator>(token.value)) {
      const auto op = std::get<Operator>(token.value);
      return getOperatorMetadata(op).isUnary ? std::nullopt : std::make_optional(op);
    }

    if (std::holds_alternative<SpecialCharacter>(token.value)) {
      switch (std::get<SpecialCharacter>(token.value)) {
        case SpecialCharacter::OpenBracket:
          return Operator::Call;
        case SpecialCharacter::OpenSquareBracket:
          return Operator::Index;
        default:
          return std::nullopt;
      }
    }

    return std::nullopt;
  }
}
