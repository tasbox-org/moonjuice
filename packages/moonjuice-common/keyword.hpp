#pragma once

namespace MoonJuice {
  enum class Keyword : uint8_t {
    Break,
    Continue,
    Return,

    Do,
    End,

    Function,

    If,
    Then,
    Else,
    ElseIf,

    For,
    In,

    Constant,
    Mutable,
    Export,
  };
}
