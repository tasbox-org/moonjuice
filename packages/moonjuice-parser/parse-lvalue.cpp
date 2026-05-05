#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Parser::ParseResult<LValue> Parser::parseLValue() {
    if (nextIs<Lexer::Symbol>()) {
      auto node = Symbol{
        .name = std::string(consume<Lexer::Symbol>()),
      };
      node.start = lookBack().start;
      node.end = lookBack().end;

      return std::make_shared<LValueValue>(node);
    }

    if (consumeIf(SpecialCharacter::OpenCurlyBracket)) {
      auto node = TableUnpack{};
      node.start = lookBack().start;

      int64_t nextNumericIndex = 1;

      do {
        if (nextIs(SpecialCharacter::CloseCurlyBracket)) {
          break;
        }

        auto unpackElement = parseTableUnpackElement(nextNumericIndex);
        if (std::holds_alternative<Error>(unpackElement)) {
          return std::get<Error>(unpackElement);
        }

        node.elements.push_back(std::move(std::get<TableUnpackElement>(unpackElement)));
      } while (consumeIf(SpecialCharacter::Comma));

      if (!consumeIf(SpecialCharacter::CloseCurlyBracket)) {
        return errorNext("Expected '}' to close table unpack");
      }

      node.end = lookBack().end;
      return std::make_shared<LValueValue>(std::move(node));
    }

    return errorNext("Expected variable name or table unpack");
  }

  Parser::ParseResult<TableUnpackElement> Parser::parseTableUnpackElement(int64_t& nextNumericTableIndex) {
    auto node = TableUnpackElement{};
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
        auto variable = parseLValue();
        if (std::holds_alternative<Error>(variable)) {
          return std::get<Error>(variable);
        }

        node.variable = std::move(std::get<LValue>(variable));
      } else {
        auto variableNode = Symbol{ .name = std::string(keyNode.value) };
        variableNode.start = keyNode.start;
        variableNode.end = keyNode.end;

        node.variable = std::make_shared<LValueValue>(variableNode);
      }
    } else if (consumeIf(SpecialCharacter::OpenSquareBracket)) {
      auto expression = parseExpression();
      if (std::holds_alternative<Error>(expression)) {
        return std::get<Error>(expression);
      }

      if (!consumeIf(SpecialCharacter::CloseSquareBracket)) {
        return errorNext("Expected ']' to close table unpack index expression");
      }

      if (!consumeIf(Operator::Assignment)) {
        return errorNext("Expected '=' after table unpack index expression");
      }

      auto variable = parseLValue();
      if (std::holds_alternative<Error>(variable)) {
        return std::get<Error>(variable);
      }

      node.key = std::move(std::get<Expression>(expression));
      node.variable = std::move(std::get<LValue>(variable));
    } else if (nextIs<Lexer::Symbol>() || nextIs(SpecialCharacter::OpenCurlyBracket)) {
      auto keyNode = Integer{ .value = nextNumericTableIndex++ };
      keyNode.start = peek().start;

      auto variable = parseLValue();
      if (std::holds_alternative<Error>(variable)) {
        return std::get<Error>(variable);
      }

      keyNode.end = lookBack().end;
      node.key = std::make_shared<ExpressionValue>(keyNode);
      node.variable = std::move(std::get<LValue>(variable));
    } else {
      return errorNext("Expected variable name, table unpack, '.<key>' or '[<key>] in unpack");
    }

    node.end = lookBack().end;
    return std::move(node);
  }
}
