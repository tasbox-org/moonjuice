#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Statement Parser::parseBreak() {
    consume<Keyword>();

    auto node = Break{};
    node.start = lookBack().start;
    node.end = lookBack().end;

    return std::make_unique<StatementValue>(node);
  }
}
