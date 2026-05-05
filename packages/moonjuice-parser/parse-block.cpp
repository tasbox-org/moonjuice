#include "src/parser.hpp"

namespace MoonJuice::Parser {
  Block Parser::parseBlock(const std::function<bool()>& hasRemaining) {
    Block block{};
    block.start = peek().start;

    while (hasRemaining()) {
      if (nextIs(Keyword::Constant) || nextIs(Keyword::Mutable) || nextIs(Keyword::Export)) {
        auto result = parseDefinition();

        handleResult<Statement>(result, [&block](Statement& statement) {
          block.expressions.emplace_back(std::move(statement));
        });
      } else if (nextIs(Keyword::Return)) {
        auto result = parseReturn();

        handleResult<Statement>(result, [&block](Statement& statement) {
          block.expressions.emplace_back(std::move(statement));
        });
      } else if (nextIs(Keyword::Break)) {
        block.expressions.emplace_back(parseBreak());
      } else {
        // TODO #156: Handle multiple return
        auto result = parseExpression();

        handleResult<Expression>(result, [&block](Expression& expression) {
          block.expressions.emplace_back(std::move(expression));
        });
      }
    }

    block.end = lookBack().end;
    return std::move(block);
  }
}
