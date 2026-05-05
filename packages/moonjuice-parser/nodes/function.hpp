#pragma once

#include "block.hpp"
#include "lvalue.hpp"

namespace MoonJuice::Parser {
  struct Function : NodeBase {
    std::vector<LValue> parameters;
    Block body;
  };
}
