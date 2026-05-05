#pragma once

namespace MoonJuice::Parser {
  struct Symbol;
  struct TableUnpack;

  using LValueValue = std::variant<Symbol, TableUnpack>;

  using LValue = std::shared_ptr<LValueValue>;
}
