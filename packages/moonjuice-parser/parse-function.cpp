#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Parser::ParseResult<Expression> Parser::parseFunction() {
    auto node = Function{};
    node.start = lookBack().start;

    if (!consumeIf(SpecialCharacter::OpenBracket)) {
      return errorNext("Expected '(' to open function parameter list");
    }

    if (!nextIs(SpecialCharacter::CloseBracket)) {
      do {
        auto parameter = parseLValue();
        if (std::holds_alternative<Error>(parameter)) {
          return std::get<Error>(parameter);
        }

        node.parameters.push_back(std::move(std::get<LValue>(parameter)));
      } while (consumeIf(SpecialCharacter::Comma));
    }

    if (!consumeIf(SpecialCharacter::CloseBracket)) {
      return errorNext("Expected ')' to close function parameter list");
    }

    node.body = parseBlock([this] { return hasRemaining() && !nextIs(Keyword::End); });
    if (!consumeIf(Keyword::End)) {
      return errorNext("Expected 'end' to close function body");
    }

    node.end = lookBack().end;
    return std::make_shared<ExpressionValue>(std::move(node));
  }
}
