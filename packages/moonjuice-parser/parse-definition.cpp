#include "src/parser.hpp"
#include "nodes/definition.hpp"

namespace MoonJuice::Parser {
  Parser::ParseResult<Statement> Parser::parseDefinition() {
    const auto keyword = consume<Keyword>();
    auto node = Definition{
      .isConstant = keyword != Keyword::Mutable,
      .isExport = keyword == Keyword::Export,
    };

    do {
      auto lhs = parseLValue();
      if (std::holds_alternative<Error>(lhs)) {
        return std::get<Error>(lhs);
      }

      node.lhs.push_back(std::move(std::get<LValue>(lhs)));
    } while (consumeIf(SpecialCharacter::Comma));

    if (!consumeIf(Operator::Assignment)) {
      return errorNext("Expected assignment operator");
    }
    node.start = lookBack().start;
    node.end = lookBack().end;

    do {
      auto rhs = parseExpression();
      if (std::holds_alternative<Error>(rhs)) {
        return std::get<Error>(rhs);
      }

      node.rhs.push_back(std::move(std::get<Expression>(rhs)));
    } while (consumeIf(SpecialCharacter::Comma));

    return std::make_unique<StatementValue>(std::move(node));
  }
}
