#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Parser::ParseResult<Expression> Parser::parseIf() {
    auto node = If{
      .elseBranch = std::nullopt,
    };
    node.start = lookBack().start;

    do {
      auto branch = IfBranch{};
      branch.start = lookBack().start;

      auto condition = parseExpression();
      if (std::holds_alternative<Error>(condition)) {
        return std::move(condition);
      }

      branch.condition = std::move(std::get<Expression>(condition));

      if (!consumeIf(Keyword::Then)) {
        return errorNext("Expected 'then' after condition");
      }

      branch.body = parseBlock([this] {
        return hasRemaining() && !nextIs(Keyword::End) && !nextIs(Keyword::Else) && !nextIs(Keyword::ElseIf);
      });
      if (!nextIs<Keyword>()) {
        return errorNext("Expected 'end', 'else' or 'elseif'");
      }

      branch.end = lookBack().end;
      node.ifBranches.push_back(std::move(branch));
    } while (consumeIf(Keyword::ElseIf));

    if (consumeIf(Keyword::Else)) {
      node.elseBranch = parseBlock([this] { return hasRemaining() && !nextIs(Keyword::End); });
    }

    if (!consumeIf(Keyword::End)) {
      return errorNext("Expected 'end' to close if expression");
    }

    node.end = lookBack().end;
    return std::make_shared<ExpressionValue>(std::move(node));
  }
}
