#pragma once

#include "../keyword.hpp"
#include "../operator.hpp"
#include "../position.hpp"
#include "../special-character.hpp"

namespace MoonJuice::Lexer {
  struct Eof {};

  struct Nil {};

  struct Symbol final : std::string_view {};

  struct Comment final : std::string_view {};

  struct String final : std::string_view {};

  struct Token {
    using Value =
      std::variant<Eof, Nil, bool, int64_t, double, String, Symbol, Keyword, Operator, SpecialCharacter, Comment>;

    Value value;
    std::string_view lexeme;

    Position start;
    Position end;
  };
}
