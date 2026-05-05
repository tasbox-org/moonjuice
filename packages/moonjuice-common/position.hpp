#pragma once

#include "shared/lua/types/serialisable.hpp"

namespace MoonJuice {
  struct Position {
    size_t line = 0;
    size_t column = 0;

    LUA_SERIALISERS(Position, "MoonJuicePosition")
  };

  ASSERT_LUA_SERIALISABLE(Position);
}
