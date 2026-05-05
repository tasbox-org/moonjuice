#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Parser::ParseResult<Expression> Parser::parseTableDefinition() {
    auto node = TableDefinition{};
    node.start = lookBack().start;

    int64_t nextNumericIndex = 1;

    do {
      if (nextIs(SpecialCharacter::CloseCurlyBracket)) {
        break;
      }

      auto definitionElement = parseTableDefinitionElement(nextNumericIndex);
      if (std::holds_alternative<Error>(definitionElement)) {
        return std::get<Error>(definitionElement);
      }

      node.elements.push_back(std::move(std::get<TableDefinitionElement>(definitionElement)));
    } while (consumeIf(SpecialCharacter::Comma));

    if (!consumeIf(SpecialCharacter::CloseCurlyBracket)) {
      return errorNext("Expected '}' to close table definition");
    }

    node.end = lookBack().end;
    return std::make_shared<ExpressionValue>(std::move(node));
  }

  Parser::ParseResult<TableDefinitionElement> Parser::parseTableDefinitionElement(int64_t& nextNumericTableIndex) {
    auto node = TableDefinitionElement{};
    node.start = peek().start;

    if (consumeIf(Operator::Index)) {
      if (!nextIs<Lexer::Symbol>()) {
        return errorNext("Expected key name");
      }

      auto keyNode = String{
        .value = consume<Lexer::Symbol>(),
      };
      keyNode.start = lookBack().start;
      keyNode.end = lookBack().end;

      node.key = std::make_shared<ExpressionValue>(keyNode);

      if (consumeIf(Operator::Assignment)) {
        auto value = parseExpression();
        if (std::holds_alternative<Error>(value)) {
          return std::get<Error>(value);
        }

        node.value = std::move(std::get<Expression>(value));
      } else if (nextIs(SpecialCharacter::Comma) || nextIs(SpecialCharacter::CloseCurlyBracket)) {
        auto value = Symbol{
          .name = std::string(keyNode.value),
        };
        value.start = keyNode.start;
        value.end = keyNode.end;

        node.value = std::make_shared<ExpressionValue>(std::move(value));
      } else {
        return errorNext("Expected '=', ',' or '}' after table key");
      }
    } else if (consumeIf(SpecialCharacter::OpenSquareBracket)) {
      auto expression = parseExpression();
      if (std::holds_alternative<Error>(expression)) {
        return std::get<Error>(expression);
      }

      if (!consumeIf(SpecialCharacter::CloseSquareBracket)) {
        return errorNext("Expected ']' to close table key expression");
      }

      if (!consumeIf(Operator::Assignment)) {
        return errorNext("Expected '=' after table key");
      }

      auto value = parseExpression();
      if (std::holds_alternative<Error>(value)) {
        return std::get<Error>(value);
      }

      node.key = std::move(std::get<Expression>(expression));
      node.value = std::move(std::get<Expression>(value));
    } else {
      auto keyNode = Integer{ .value = nextNumericTableIndex++ };
      keyNode.start = peek().start;

      auto value = parseExpression();
      if (std::holds_alternative<Error>(value)) {
        return std::get<Error>(value);
      }

      keyNode.end = lookBack().end;
      node.key = std::make_shared<ExpressionValue>(keyNode);
      node.value = std::move(std::get<Expression>(value));
    }

    node.end = lookBack().end;
    return std::move(node);
  }
}
