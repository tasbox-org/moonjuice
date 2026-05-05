#pragma once

#include "expression.hpp"

namespace MoonJuice::Parser {
  struct TableDefinitionElement : NodeBase {
    Expression key;
    Expression value;
  };

  struct TableDefinition : NodeBase {
    std::vector<TableDefinitionElement> elements;
  };
}
