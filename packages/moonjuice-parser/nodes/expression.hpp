#pragma once

namespace MoonJuice::Parser {
  struct Nil;
  struct Boolean;
  struct Integer;
  struct Number;
  struct String;
  struct TableDefinition;
  struct Symbol;
  struct Block;

  struct UnaryOperator;
  struct BinaryOperator;
  struct Call;

  struct Function;
  struct If;
  struct For;

  using ExpressionValue = std::variant<
    Nil,
    Boolean,
    Integer,
    Number,
    String,
    TableDefinition,
    Symbol,
    Block,
    UnaryOperator,
    BinaryOperator,
    Function,
    If,
    For,
    Call>;

  using Expression = std::shared_ptr<ExpressionValue>;
}
