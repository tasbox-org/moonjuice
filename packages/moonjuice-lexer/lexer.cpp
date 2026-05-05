#include "lexer.hpp"
#include "shared/helpers/lambda-overload.hpp"
#include <format>

namespace TASBox::Language::Lexer {
  Lexer::Lexer(std::string_view sourceCode)
    : stream(PeekableStream(sourceCode.cbegin(), sourceCode.cend())) {
    binaryLexer = NumeralLexer{
      .isDigit = [](const char c) {
        return c >= '0' && c <= '1';
      },
      .supportsFloat = false,
    };

    decimalLexer = NumeralLexer{
      .isDigit = [](const char c) {
        return c >= '0' && c <= '9';
      },
      .supportsFloat = true,
    };

    hexLexer = NumeralLexer{
      .isDigit =
      [](const char c) {
        return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F');
      },
      .supportsFloat = false,
    };

    symbolToSpecialToken = std::map<std::string_view, Token::Value>{
      { "break", Keyword::Break },
      { "continue", Keyword::Continue },
      { "return", Keyword::Return },

      { "do", Keyword::Do },
      { "end", Keyword::End },

      { "fn", Keyword::Function },

      { "if", Keyword::If },
      { "then", Keyword::Then },
      { "else", Keyword::Else },
      { "elseif", Keyword::ElseIf },

      { "for", Keyword::For },
      { "in", Keyword::In },

      { "def", Keyword::Constant },
      { "mut", Keyword::Mutable },
      { "export", Keyword::Export },

      { "true", true },
      { "false", false },

      { "not", Operator::Not },
      { "and", Operator::And },
      { "or", Operator::Or },

      { "nil", Nil{} },
    };
  }

  std::vector<Token> Lexer::tokenise() {
    std::vector<Token> tokens;

    while (stream.hasRemaining()) {
      startIterator = stream.getCurrent();
      startPosition = currentPosition;

      if (isWhitespace(peek())) {
        do {
          consume();
        } while (isWhitespace(peek()));
      } else {
        auto result = tokeniseNext();

        std::visit(
          LambdaOverload{
            [&tokens](const Token& token) {
              tokens.push_back(token);
            },
            [this](const Error& error) {
              errors.push_back(error);
            },
          },
          result
        );
      }
    }

    return std::move(tokens);
  }

  const std::vector<Error>& Lexer::getErrors() const {
    return errors;
  }

  Lexer::TokeniseResult Lexer::tokeniseNext() {
    const auto next = peek();

    if (next == '-' && peek(1) == '-') {
      return tokeniseComment();
    }

    if (decimalLexer.isDigit(next)) {
      return tokeniseNumber();
    }

    if (isSymbolOpener(next)) {
      return tokeniseSymbol();
    }

    if (isOperator(next)) {
      return tokeniseOperator();
    }

    if (isString(next)) {
      return tokeniseString();
    }

    if (isSpecialCharacter(next)) {
      return tokeniseSpecialCharacter();
    }

    consume();
    return error("Unexpected character");
  }

  Lexer::TokeniseResult Lexer::tokeniseNumber() {
    auto isFloat = false;
    auto requiresDigit = true;
    const auto& numeral = getNumeralLexer();
    std::string convertableString;

    while (numeral.isDigit(peek())) {
      convertableString += consume();

      if (peek() == '_') {
        consume();
        requiresDigit = true;
      } else if (peek() == '.') {
        if (!numeral.supportsFloat) {
          return error("Malformed number: Hex and binary do not support floating point");
        }

        if (isFloat) {
          return error("Malformed number: More than one decimal point");
        }

        isFloat = true;
        requiresDigit = true;
        convertableString += consume();
      } else {
        requiresDigit = false;
      }
    }

    if (requiresDigit) {
      return error("Malformed number: A digit must follow '0x', '0b', '_' and '.'");
    }

    return token(isFloat ? Token::Value(std::stod(convertableString)) : Token::Value(std::stol(convertableString)));
  }

  Lexer::TokeniseResult Lexer::tokeniseSymbol() {
    while (isSymbol(peek())) {
      consume();
    }

    const auto symbol = lexeme();

    if (symbolToSpecialToken.contains(symbol)) {
      return token(symbolToSpecialToken.at(symbol));
    }

    return token(Symbol(symbol));
  }

  Lexer::TokeniseResult Lexer::tokeniseOperator() {
    switch (consume()) {
      case '+':
        return token(Operator::Add);
      case '-':
        return token(Operator::Subtract);
      case '*':
        return token(Operator::Multiply);
      case '/':
        return token(Operator::Divide);
      case '%':
        return token(Operator::Modulo);
      case '=':
        return token(consumeIf('=') ? Operator::Equals : Operator::Assignment);
      case '~':
        return token(consumeIf('=') ? Operator::NotEquals : Operator::BitwiseNot);
      case '<':
        if (consumeIf('<')) {
          return token(Operator::LeftShift);
        }

        return token(consumeIf('=') ? Operator::LessThanOrEqual : Operator::LessThan);
      case '>':
        if (consumeIf('>')) {
          return token(Operator::RightShift);
        }

        return token(consumeIf('=') ? Operator::GreaterThanOrEqual : Operator::GreaterThan);
      case '|':
        return token(consumeIf('>') ? Operator::Pipe : Operator::BitwiseOr);
      case '.':
        return token(consumeIf('.') ? Operator::Concat : Operator::Index);
      case '&':
        return token(Operator::BitwiseAnd);
      case '^':
        return token(Operator::BitwiseXor);
      case '#':
        return token(Operator::Length);
      case '?':
        if (consumeIf('?')) {
          return token(Operator::OptionalCoalesce);
        }

        if (consumeIf('.')) {
          if (consumeIf('(')) {
            return token(Operator::OptionalCall);
          }

          if (consumeIf('[')) {
            return token(Operator::OptionalIndexExpression);
          }

          return token(Operator::OptionalIndex);
        }

        return error("Unexpected character following '?'. Expected '?.', '?.()', '?.[]' or '\?\?' operators");
      default:
        throw std::runtime_error("tokeniseOperator called without checking isOperator");
    }
  }

  // TODO #156: Properly handle string escapes (this doesn't allow e.g. escaping a backslash at the end of a string)
  Lexer::TokeniseResult Lexer::tokeniseString() {
    const auto delimiter = consume();

    while (!stream.consumeIf(
      [&delimiter](const char c) {
        return c == delimiter;
      }
    )) {
      if (peek() == '\\' && peek(1) == delimiter) {
        consume(2);
      } else {
        consume();
      }

      if (!stream.hasRemaining()) {
        return error("A string ran off the end of the program");
      }
    }

    return token(String(std::string_view{ startIterator + 1, stream.getCurrent() - 1 }));
  }

  Lexer::TokeniseResult Lexer::tokeniseSpecialCharacter() {
    switch (consume()) {
      case '(':
        return token(SpecialCharacter::OpenBracket);
      case ')':
        return token(SpecialCharacter::CloseBracket);
      case '[':
        return token(SpecialCharacter::OpenSquareBracket);
      case ']':
        return token(SpecialCharacter::CloseSquareBracket);
      case '{':
        return token(SpecialCharacter::OpenCurlyBracket);
      case '}':
        return token(SpecialCharacter::CloseCurlyBracket);
      case ',':
        return token(SpecialCharacter::Comma);
      case ':':
        return token(SpecialCharacter::Colon);
      default:
        throw std::runtime_error("tokeniseSpecialCharacter called without checking isSpecialCharacter");
    }
  }

  Lexer::TokeniseResult Lexer::tokeniseComment() {
    consume(2);

    if (peek() == '[' && peek(1) == '[') {
      consume(2);

      while (peek() != '-' || peek(1) != '-' || peek(2) != ']' || peek(3) != ']') {
        if (!stream.hasRemaining(5)) {
          return error("Missing closing '--]]' for multiline comment");
        }

        consume();
      }

      consume(4);

      return token(Comment(std::string_view{ startIterator + 4, stream.getCurrent() - 4 }));
    } else {
      while (peek() != '\n') {
        consume();
      }

      return token(Comment(std::string_view{ startIterator + 2, stream.getCurrent() }));
    }
  }

  const Lexer::NumeralLexer& Lexer::getNumeralLexer() {
    if (peek() == '0' && peek(1) == 'b') {
      consume(2);
      return binaryLexer;
    }

    if (peek() == '0' && peek(1) == 'x') {
      consume(2);
      return hexLexer;
    }

    return decimalLexer;
  }

  Token Lexer::token(const Token::Value& value) const {
    return {
      .value = value,
      .lexeme = lexeme(),
      .start = startPosition,
      .end = currentPosition,
    };
  }

  Error Lexer::error(const std::string_view message) const {
    return {
      .message = std::string(message),
      .start = startPosition,
      .end = currentPosition,
    };
  }

  char Lexer::consume() {
    const char ch = stream.consume();

    if (ch == '\n') {
      currentPosition.column = 0;
      currentPosition.line++;
    } else {
      currentPosition.column++;
    }

    return ch;
  }

  void Lexer::consume(const size_t amount) {
    for (size_t i = 0; i < amount; i++) {
      consume();
    }
  }

  bool Lexer::consumeIf(char c) {
    return stream.consumeIf(
      [c](const char& next) {
        return next == c;
      }
    );
  }

  char Lexer::peek(const size_t distance) const {
    if (stream.hasRemaining(distance + 1)) {
      return stream.peek(distance);
    }

    return '\0';
  }

  std::string_view Lexer::lexeme() const {
    return { startIterator, stream.getCurrent() };
  }

  bool Lexer::isLetter(const char c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
  }

  bool Lexer::isSymbolOpener(const char c) {
    return isLetter(c) || c == '_';
  }

  bool Lexer::isSymbol(const char c) const {
    return isSymbolOpener(c) || decimalLexer.isDigit(c);
  }

  bool Lexer::isOperator(const char c) {
    switch (c) {
      case '+':
      case '-':
      case '*':
      case '/':
      case '%':
      case '=':
      case '~':
      case '<':
      case '>':
      case '|':
      case '.':
      case '&':
      case '^':
      case '#':
      case '?':
        return true;
      default:
        return false;
    }
  }

  bool Lexer::isString(const char c) {
    return c == '"' || c == '\'';
  }

  bool Lexer::isWhitespace(const char c) {
    return c == ' ' || c == '\t' || c == '\r' || c == '\n';
  }

  bool Lexer::isSpecialCharacter(char c) {
    return c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}' || c == ',' || c == ':';
  }
}
