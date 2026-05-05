#pragma once

namespace MoonJuice::Parser {
  struct Nil : NodeBase {};

  struct Boolean : NodeBase {
    bool value;
  };

  struct Integer : NodeBase {
    int64_t value;
  };

  struct Number : NodeBase {
    double value;
  };

  struct String : NodeBase {
    std::string_view value;
  };
}
