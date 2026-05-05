#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Parser::ParseResult<Expression> Parser::parseOperand() {
    const auto next = peek();

    return std::visit<ParseResult<Expression>>(
      LambdaOverload{
        [this](Operator) { return parseUnaryOperator(); },
        [this, &next](SpecialCharacter) -> ParseResult<Expression> {
          switch (consume<SpecialCharacter>()) {
            case SpecialCharacter::OpenBracket: {
              auto innerExpression = parseExpression();
              if (std::holds_alternative<Error>(innerExpression)) {
                return std::move(innerExpression);
              }

              if (!consumeIf(SpecialCharacter::CloseBracket)) {
                return errorNext("Expected ')' to close expression");
              }

              return std::move(std::get<Expression>(innerExpression));
            }
            case SpecialCharacter::OpenCurlyBracket:
              return parseTableDefinition();
            default:
              return error("Unexpected special character in expression", next.start, next.end);
          }
        },
        [this, &next](Keyword) -> ParseResult<Expression> {
          switch (consume<Keyword>()) {
            case Keyword::Do: {
              auto block = parseBlock([this] { return hasRemaining() && !nextIs(Keyword::End); });
              if (!consumeIf(Keyword::End)) {
                return errorNext("Expected 'end' to close 'do' block");
              }

              return std::make_shared<ExpressionValue>(std::move(block));
            }
            case Keyword::Function:
              return parseFunction();
            case Keyword::If:
              return parseIf();
            case Keyword::For:
              return parseFor();
            default:
              return error("Unexpected keyword in expression", next.start, next.end);
          }
        },
        [this, &next](Lexer::Nil) {
          consume();
          auto node = Nil{};
          node.start = next.start;
          node.end = next.end;

          return std::make_shared<ExpressionValue>(node);
        },
        [this, &next](const bool b) {
          consume();
          auto node = Boolean{.value = b};
          node.start = next.start;
          node.end = next.end;

          return std::make_shared<ExpressionValue>(node);
        },
        [this, &next](const int64_t integer) {
          consume();
          auto node = Integer{.value = integer};
          node.start = next.start;
          node.end = next.end;

          return std::make_shared<ExpressionValue>(node);
        },
        [this, &next](const double number) {
          consume();
          auto node = Number{.value = number};
          node.start = next.start;
          node.end = next.end;

          return std::make_shared<ExpressionValue>(node);
        },
        [this, &next](const Lexer::String& string) {
          consume();
          auto node = String{.value = string};
          node.start = next.start;
          node.end = next.end;

          return std::make_shared<ExpressionValue>(node);
        },
        [this, &next](const Lexer::Symbol& symbol) {
          consume();
          auto node = Symbol{.name = std::string(symbol)};
          node.start = next.start;
          node.end = next.end;

          return std::make_shared<ExpressionValue>(node);
        },
        [this, &next](Lexer::Eof) { return error("Expression ran off the end of the program", next.start, next.end); },
        [](Lexer::Comment) -> ParseResult<Expression> {
          throw std::runtime_error("Comment was not stripped from tokens before parsing");
        }
      },
      next.value
    );
  }
}
