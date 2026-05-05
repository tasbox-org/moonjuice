#pragma once

#include "block.hpp"
#include "expression.hpp"

namespace MoonJuice::Parser {
  struct IfBranch : NodeBase {
    Expression condition;
    Block body;
  };

  struct If : NodeBase {
    std::vector<IfBranch> ifBranches;
    std::optional<Block> elseBranch;
  };
}
