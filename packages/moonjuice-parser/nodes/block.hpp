#pragma once

#include "expression.hpp"
#include "node-base.hpp"
#include "statement.hpp"

namespace MoonJuice::Parser {
  struct Block : NodeBase {
    std::vector<std::variant<Expression, Statement>> expressions;
  };
}
