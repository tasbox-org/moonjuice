#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Parser::ParseResult<Expression> Parser::parseFor() {
    auto node = For{};
    node.start = lookBack().start;

    do {
      auto lhs = parseLValue();
      if (std::holds_alternative<Error>(lhs)) {
        return std::get<Error>(lhs);
      }

      node.lhs.push_back(std::move(std::get<LValue>(lhs)));
    } while (consumeIf(SpecialCharacter::Comma));

    if (!consumeIf(Keyword::In)) {
      return errorNext("Expected 'in' to follow for loop variable definitions");
    }

    auto enumerable = parseExpression();
    if (std::holds_alternative<Error>(enumerable)) {
      return std::get<Error>(enumerable);
    }

    node.enumerable = std::move(std::get<Expression>(enumerable));

    if (!consumeIf(Keyword::Do)) {
      return errorNext("Expected 'do' to follow for loop enumerable");
    }

    node.body = parseBlock(
      [this] {
        return hasRemaining() && !nextIs(Keyword::End);
      }
    );
    if (!consumeIf(Keyword::End)) {
      return errorNext("Expected 'end' to close for loop body");
    }

    node.end = lookBack().end;
    return std::make_shared<ExpressionValue>(std::move(node));
  }
}
