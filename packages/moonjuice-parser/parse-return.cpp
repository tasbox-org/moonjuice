#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Parser::ParseResult<Statement> Parser::parseReturn() {
    consume<Keyword>();

    auto node = Return{};
    node.start = lookBack().start;

    auto valueResult = parseExpression();
    if (std::holds_alternative<Error>(valueResult)) {
      return std::get<Error>(valueResult);
    }

    node.value = std::move(std::get<Expression>(valueResult));
    node.end = lookBack().end;

    return std::make_unique<StatementValue>(std::move(node));
  }
}
