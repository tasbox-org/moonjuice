#include "parser.hpp"

namespace TASBox::Language::Parser {
  Parser::Parser(std::span<const Lexer::Token> tokens) : stream(this->tokens.begin(), this->tokens.end()) {
    this->tokens.reserve(tokens.size());
    std::ranges::copy_if(tokens.begin(), tokens.end(), std::back_inserter(this->tokens), [](auto& token) {
      return !std::holds_alternative<Lexer::Comment>(token.value);
    });

    stream = PeekableStream(this->tokens.begin(), this->tokens.end());
  }

  Block Parser::parse() {
    return parseBlock([this] { return hasRemaining(); });
  }

  const std::vector<Error>& Parser::getErrors() const {
    return errors;
  }

  Error Parser::error(const std::string_view message, const Position start, const Position end) {
    return Error{
      .message = std::string(message),
      .start = start,
      .end = end,
    };
  }

  Error Parser::errorNext(const std::string_view message) const {
    return error(message, peek().start, peek().end);
  }

  Lexer::Token Parser::consume() {
    return stream.consume();
  }

  Lexer::Token Parser::peek(const size_t distance) const {
    if (stream.hasRemaining(distance + 1)) {
      return stream.peek(distance);
    }

    const auto previousToken = stream.lookBack({
      .start = Position{.line = 0, .column = 0},
      .end = Position{.line = 0, .column = 0},
    });

    return {
      .value = Lexer::Eof{},
      .lexeme = "",
      .start = previousToken.start,
      .end = previousToken.end,
    };
  }

  Lexer::Token Parser::lookBack() const {
    return stream.lookBack({
      .value = Lexer::Eof{},
      .lexeme = "",
      .start = Position{.line = 0, .column = 0},
      .end = Position{.line = 0, .column = 0},
    });
  }

  bool Parser::hasRemaining() const {
    return stream.hasRemaining();
  }
}
