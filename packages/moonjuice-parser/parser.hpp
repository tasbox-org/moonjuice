#pragma once

#include <functional>

#include "nodes.hpp"
#include "shared/helpers/lambda-overload.hpp"
#include "shared/helpers/peekable-stream.hpp"
#include "shared/language/error.hpp"
#include "shared/language/lexer/token.hpp"

namespace MoonJuice::Parser {
  class Parser {
  public:
    using Precedence = uint16_t;

    explicit Parser(std::span<const Lexer::Token> tokens);

    [[nodiscard]] Block parse();

    [[nodiscard]] const std::vector<Error>& getErrors() const;

  private:
    using Iterator = std::vector<Lexer::Token>::iterator;
    template<typename T>
    using ParseResult = std::variant<T, Error>;

    struct OperatorMetadata {
      bool isUnary = false;
      Precedence precedence = 0;
      bool isRightAssociative = false;
    };

    std::vector<Lexer::Token> tokens;
    PeekableStream<Iterator> stream;
    std::vector<Error> errors;

    [[nodiscard]] ParseResult<Statement> parseDefinition();

    [[nodiscard]] ParseResult<Statement> parseReturn();

    [[nodiscard]] Statement parseBreak();

    [[nodiscard]] ParseResult<Expression> parseExpression();

    [[nodiscard]] ParseResult<Expression> parseExpression(Expression lhs, Precedence minPrecedence);

    [[nodiscard]] ParseResult<Expression> parseOperand();

    [[nodiscard]] Block parseBlock(const std::function<bool()>& hasRemaining);

    [[nodiscard]] ParseResult<Expression> parseUnaryOperator();

    [[nodiscard]] ParseResult<LValue> parseLValue();

    [[nodiscard]] ParseResult<TableUnpackElement> parseTableUnpackElement(int64_t& nextNumericTableIndex);

    [[nodiscard]] ParseResult<Expression> parseFor();

    [[nodiscard]] ParseResult<Expression> parseFunction();

    [[nodiscard]] ParseResult<Expression> parseIf();

    [[nodiscard]] ParseResult<Expression> parseTableDefinition();

    [[nodiscard]] ParseResult<TableDefinitionElement> parseTableDefinitionElement(int64_t& nextNumericTableIndex);

    [[nodiscard]] ParseResult<Expression> parseBinaryOperator(Expression lhs);

    [[nodiscard]] ParseResult<Expression> parseCall(Expression lhs);

    [[nodiscard]] ParseResult<Expression> parseIndexExpression(Expression lhs);

    [[nodiscard]] static OperatorMetadata getOperatorMetadata(Operator op);

    [[nodiscard]] static std::optional<Operator> getIfSubExpressionBinaryOperator(const Lexer::Token& token);

    [[nodiscard]] static bool isBinaryOperatorWithGreaterOrEqualPrecedence(
      const Lexer::Token& token,
      Precedence minPrecedence
    );

    [[nodiscard]] static bool isRightHandSideChainedOperator(
      const Lexer::Token& token,
      Precedence previousOperatorPrecedence
    );

    [[nodiscard]] static Error error(std::string_view message, Position start, Position end);

    [[nodiscard]] Error errorNext(std::string_view message) const;

    Lexer::Token consume();

    template<typename T>
    T consume() {
      return std::get<T>(stream.consume().value);
    }

    template<typename T>
    bool consumeIf(T value) {
      if (nextIs<T>(value)) {
        consume();
        return true;
      }

      return false;
    }

    template<typename T>
    [[nodiscard]] bool nextIs() const {
      return std::holds_alternative<T>(peek().value);
    }

    template<typename T>
    [[nodiscard]] bool nextIs(T value) const {
      const auto next = peek();

      return static_cast<bool>(std::holds_alternative<T>(next.value) && std::get<T>(next.value) == value);
    }

    [[nodiscard]] Lexer::Token peek(size_t distance = 0) const;

    template<typename T>
    T peek() {
      return std::get<T>(stream.peek().value);
    }

    [[nodiscard]] Lexer::Token lookBack() const;

    [[nodiscard]] bool hasRemaining() const;

    template<typename T>
    void handleResult(ParseResult<T>& result, const std::function<void(T& value)>& ifOk) {
      std::visit(
        LambdaOverload{
          ifOk,
          [this](const Error& error) {
            errors.push_back(error);
          },
        },
        result
      );
    }
  };
}
