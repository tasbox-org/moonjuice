#pragma once

#include "expression.hpp"
#include "node-base.hpp"

namespace MoonJuice::Parser {
  struct Return : NodeBase {
    Expression value;
  };
}
