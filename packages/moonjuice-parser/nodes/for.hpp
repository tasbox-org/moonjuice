#pragma once

#include "block.hpp"
#include "expression.hpp"
#include "lvalue.hpp"

namespace MoonJuice::Parser {
  struct For : NodeBase {
    std::vector<LValue> lhs;
    Expression enumerable;

    Block body;
  };
}
