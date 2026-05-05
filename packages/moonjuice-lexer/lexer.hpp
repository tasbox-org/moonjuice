#pragma once

#include <functional>
#include "src/token.hpp"
#include "../error.hpp"
#include "shared/helpers/peekable-stream.hpp"

namespace MoonJuice::Lexer {
  class Lexer {
  public:
    explicit Lexer(std::string_view sourceCode);

    [[nodiscard]] std::vector<Token> tokenise();

    [[nodiscard]] const std::vector<Error>& getErrors() const;

  private:
    struct NumeralLexer {
      std::function<bool(char)> isDigit;
      bool supportsFloat;
    };

    using TokeniseResult = std::variant<Token, Error>;

    PeekableStream<std::string_view::const_iterator> stream;

    std::string_view::const_iterator startIterator{};

    Position startPosition{};
    Position currentPosition{};

    std::vector<Error> errors;

    NumeralLexer binaryLexer;
    NumeralLexer decimalLexer;
    NumeralLexer hexLexer;

    std::map<std::string_view, Token::Value> symbolToSpecialToken;

    [[nodiscard]] TokeniseResult tokeniseNext();

    [[nodiscard]] TokeniseResult tokeniseNumber();

    [[nodiscard]] TokeniseResult tokeniseSymbol();

    [[nodiscard]] TokeniseResult tokeniseOperator();

    [[nodiscard]] TokeniseResult tokeniseString();

    [[nodiscard]] TokeniseResult tokeniseSpecialCharacter();

    [[nodiscard]] TokeniseResult tokeniseComment();

    [[nodiscard]] const NumeralLexer& getNumeralLexer();

    [[nodiscard]] Token token(const Token::Value& value) const;

    [[nodiscard]] Error error(std::string_view message) const;

    char consume();

    void consume(size_t amount);

    bool consumeIf(char c);

    [[nodiscard]] char peek(size_t distance = 0) const;

    [[nodiscard]] std::string_view lexeme() const;

    [[nodiscard]] static bool isLetter(char c);

    [[nodiscard]] static bool isSymbolOpener(char c);

    [[nodiscard]] bool isSymbol(char c) const;

    [[nodiscard]] static bool isOperator(char c);

    [[nodiscard]] static bool isString(char c);

    [[nodiscard]] static bool isWhitespace(char c);

    [[nodiscard]] static bool isSpecialCharacter(char c);
  };
}
