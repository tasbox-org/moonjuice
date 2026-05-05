#pragma once

#include <string>

#include "position.hpp"

namespace MoonJuice {
  struct Error {
    std::string message;

    Position start;
    Position end;
  };
}
