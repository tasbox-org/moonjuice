#pragma once

#include "expression.hpp"
#include "lvalue.hpp"

namespace MoonJuice::Parser {
  struct TableUnpackElement : NodeBase {
    Expression key;
    LValue variable;
  };

  struct TableUnpack : NodeBase {
    std::vector<TableUnpackElement> elements;
  };
}
