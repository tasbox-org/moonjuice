#pragma once
#include "shared/language/position.hpp"

namespace MoonJuice::Parser {
  struct NodeBase {
    Position start;
    Position end;
  };
}
