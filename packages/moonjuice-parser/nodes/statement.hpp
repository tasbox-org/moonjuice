#pragma once

#include "break.hpp"
#include "definition.hpp"
#include "return.hpp"

namespace MoonJuice::Parser {
  using StatementValue = std::variant<Definition, Return, Break>;

  using Statement = std::unique_ptr<StatementValue>;
}
