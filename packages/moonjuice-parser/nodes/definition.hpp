#pragma once

#include "expression.hpp"
#include "lvalue.hpp"
#include "node-base.hpp"

namespace MoonJuice::Parser {
  struct Definition : NodeBase {
    bool isConstant = true;
    bool isExport = false;

    std::vector<LValue> lhs;
    std::vector<Expression> rhs;
  };
}
