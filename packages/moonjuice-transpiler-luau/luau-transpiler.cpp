#include "luau-transpiler.hpp"

namespace TASBox::Language::Transpiler {
  LuauTranspiler::LuauTranspiler(bool showDebugComments) : showDebugComments(showDebugComments) {}

  std::string LuauTranspiler::transpile(const Parser::Block& block) {
    const auto exportsSymbol = "exports_" + uuidGenerator.generate().toString({ .hyphenated = false });
    out << "local " << exportsSymbol << " = {}\n\n";

    pushTopLevelScope(exportsSymbol);
    emitBlock(block, "return");
    popScope();

    out << "\nreturn " << exportsSymbol << "\n";
    return out.str();
  }

  void LuauTranspiler::emit(const Parser::Block& node) {
    if (scope().isInExpression) {
      out << "(function() ";
    } else {
      out << "do\n";
    }

    emitBlock(node, "return");

    if (scope().isInExpression) {
      out << "end)()";
    } else {
      out << "end";
    }
  }

  void LuauTranspiler::emit(const Parser::Expression& node) {
    std::visit(
      [this](auto& n) {
        emit(n);
      },
      *node
    );
  }

  void LuauTranspiler::emit(const Parser::Statement& node) {
    std::visit(
      [this](auto& n) {
        emit(n);
      },
      *node
    );
  }

  void LuauTranspiler::emit(const Parser::LValue& node) {
    std::visit(
      [this](auto& n) {
        emit(n);
      },
      *node
    );
  }

  void LuauTranspiler::emit(const Parser::Nil& node) {
    if (!scope().isInExpression) {
      out << "--[[ elided unused nil ]]";
      return;
    }

    out << "nil";
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::Boolean& node) {
    if (!scope().isInExpression) {
      out << "--[[ elided unused boolean ]]";
      return;
    }

    out << (node.value ? "true" : "false");
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::Integer& node) {
    if (!scope().isInExpression) {
      out << "--[[ elided unused integer ]]";
      return;
    }

    out << node.value;
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::Number& node) {
    if (!scope().isInExpression) {
      out << "--[[ elided unused number ]]";
      return;
    }

    out << node.value;
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::String& node) {
    if (!scope().isInExpression) {
      out << "--[[ elided unused string ]]";
      return;
    }

    out << "'" << node.value << "'"; // TODO #156: Sanitise
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::TableDefinition& node) {
    if (!scope().isInExpression) {
      out << "local _ = ";
    }

    pushExpressionScope();

    out << "{ ";
    emitCommaSeparated(node.elements);
    out << " }";

    popScope();
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::TableDefinitionElement& node) {
    out << "[";
    emit(node.key);
    out << "] = ";
    emit(node.value);
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::Symbol& node) {
    if (!scope().isInExpression && !scope().isInLValue) {
      out << "--[[ elided unused symbol ]]";
      return;
    }

    out << node.name;
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::UnaryOperator& node) {
    if (!scope().isInExpression) {
      out << "local _ = ";
    }

    pushExpressionScope();

    switch (node.op) {
      case Operator::Subtract:
        out << "(-";
        emit(node.rhs);
        out << ")";

        break;
      case Operator::Not:
        out << "(not ";
        emit(node.rhs);
        out << ")";

        break;
      case Operator::BitwiseNot:
        out << "bit32.bnot(";
        emit(node.rhs);
        out << ")";

        break;
      case Operator::Length:
        out << "(#";
        emit(node.rhs);
        out << ")";

        break;
      default:
        throw std::runtime_error("Invalid AST. UnaryOperator node contains a non-unary Operator");
    }

    popScope();
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::BinaryOperator& node) {
    // Special handling of assignment operator which must either be a statement or wrapped in a function
    if (node.op == Operator::Assignment) {
      emitAssignmentOperator(node);
      return;
    }

    if (!scope().isInExpression) {
      out << "local _ = ";
    }

    pushExpressionScope();

    switch (node.op) {
      case Operator::Add:
        emitRegularOperator(node, "+");
        break;
      case Operator::Subtract:
        emitRegularOperator(node, "-");
        break;
      case Operator::Multiply:
        emitRegularOperator(node, "*");
        break;
      case Operator::Divide:
        emitRegularOperator(node, "/");
        break;
      case Operator::Modulo:
        emitRegularOperator(node, "%");
        break;
      case Operator::Concat:
        emitRegularOperator(node, "..");
        break;
      case Operator::And:
        emitRegularOperator(node, "and");
        break;
      case Operator::Or:
        emitRegularOperator(node, "or");
        break;
      case Operator::OptionalCoalesce:
        emitCoalesceOperator(node);
        break;
      case Operator::Equals:
        emitRegularOperator(node, "==");
        break;
      case Operator::NotEquals:
        emitRegularOperator(node, "~=");
        break;
      case Operator::LessThan:
        emitRegularOperator(node, "<");
        break;
      case Operator::GreaterThan:
        emitRegularOperator(node, ">");
        break;
      case Operator::LessThanOrEqual:
        emitRegularOperator(node, "<=");
        break;
      case Operator::GreaterThanOrEqual:
        emitRegularOperator(node, ">=");
        break;
      case Operator::Pipe:
        emitPipeOperator(node);
        break;
      case Operator::Index:
      case Operator::IndexExpression:
      case Operator::OptionalIndex:
      case Operator::OptionalIndexExpression:
        emitIndexOperator(node);
        break;
      case Operator::BitwiseAnd:
        emitFunctionOperator(node, "bit32.band");
        break;
      case Operator::BitwiseOr:
        emitFunctionOperator(node, "bit32.bor");
        break;
      case Operator::BitwiseXor:
        emitFunctionOperator(node, "bit32.bxor");
        break;
      case Operator::LeftShift:
        emitFunctionOperator(node, "bit32.lshift");
        break;
      case Operator::RightShift:
        emitFunctionOperator(node, "bit32.rshift");
        break;
      default:
        throw std::runtime_error("Invalid AST. Unsupported or unary operator in binary operator node");
    }

    popScope();
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::Function& node) {
    if (!scope().isInExpression) {
      out << "--[[ elided unused function definition ]]";
      return;
    }

    out << "function(";
    pushLValueScope();
    emitCommaSeparated(node.parameters);
    auto tableUnpacks = std::move(scope().tableUnpacks);
    popScope();
    out << ")\n";

    emitTableUnpacks(tableUnpacks);
    emitBlock(node.body, "return");

    out << "end";
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::If& node) {
    if (scope().isInExpression) {
      out << "(function() ";
    }

    out << "if (";

    for (const auto& branch : node.ifBranches) {
      pushExpressionScope();
      emit(branch.condition);
      popScope();
      out << ") then\n";

      emitBlock(branch.body, "return");

      if (&branch != &node.ifBranches.back()) {
        out << "elseif (";
      }
    }

    if (node.elseBranch.has_value()) {
      out << "else\n";

      emitBlock(node.elseBranch.value(), "return");
    }

    out << "end";
    if (scope().isInExpression) {
      out << " end)()";
    }

    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::For& node) {
    const auto retSymbol = "ret_" + uuidGenerator.generate().toString({ .hyphenated = false });
    const auto elementSymbol = "element_" + uuidGenerator.generate().toString({ .hyphenated = false });
    if (scope().isInExpression) {
      out << "(function()\nlocal " << retSymbol << " = {}\n";
    }

    out << "for ";

    pushLValueScope();
    emitCommaSeparated(node.lhs);
    const auto tableUnpacks = std::move(scope().tableUnpacks);
    popScope();

    out << " in ";

    pushExpressionScope();
    emit(node.enumerable);
    popScope();

    out << " do\n";
    emitTableUnpacks(tableUnpacks);
    emitBlock(node.body, "local " + elementSymbol + " = ");

    if (scope().isInExpression) {
      out << "\n";
      out << "table.insert(" << retSymbol << ", " << elementSymbol << ")\n";
    }

    out << "end";

    if (scope().isInExpression) {
      out << "\nreturn " << retSymbol << "\nend)()";
    }

    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::Call& node) {
    if (!scope().isInExpression) {
      // Lua throws for `(<expr>)(<...args>)` if not explicitly an expression
      // while `<symbol>(<...args>)` works fine
      // Not worth simplifying the output for that one case, so always force an expression
      out << "local _ = ";
    }

    pushExpressionScope();

    if (node.isOptional) {
      out << "(function() local lhs = ";
    }

    if (std::holds_alternative<Parser::Symbol>(*node.lhs)) {
      emit(node.lhs);
    } else {
      out << "(";
      emit(node.lhs);
      out << ")";
    }

    if (node.isOptional) {
      out << "; if lhs == nil then return nil else return lhs";
    }

    out << "(";
    emitCommaSeparated(node.arguments);
    out << ")";

    if (node.isOptional) {
      out << " end end)()";
    }

    popScope();
    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::Definition& node) {
    out << "local ";

    pushLValueScope();
    emitCommaSeparated(node.lhs);
    const auto tableUnpacks = std::move(scope().tableUnpacks);
    popScope();

    out << " = ";

    pushExpressionScope();
    emitCommaSeparated(node.rhs);
    popScope();

    if (!tableUnpacks.empty()) {
      out << ";\n";
      emitTableUnpacks(tableUnpacks);
    }

    if (node.isExport) {
      for (const auto& lValue : node.lhs) {
        emitExports(lValue);
      }
    }

    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::Return& node) {
    out << "return ";

    pushExpressionScope();
    emit(node.value);
    popScope();

    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::Break& node) {
    out << "break";

    emitPosition(node);
  }

  void LuauTranspiler::emit(const Parser::TableUnpack& node) {
    auto symbol = Parser::Symbol{
      .name = "tbl_" + uuidGenerator.generate().toString({ .hyphenated = false }),
    };
    symbol.start = node.start;
    symbol.end = node.end;

    out << symbol.name;
    emitPosition(symbol);

    auto unpack = Parser::Definition{};
    unpack.start = node.start;
    unpack.end = node.end;

    const auto indexLhs = std::make_shared<Parser::ExpressionValue>(symbol);
    for (const auto& element : node.elements) {
      auto index = Parser::BinaryOperator{ .op = Operator::IndexExpression, .lhs = indexLhs, .rhs = element.key };
      index.start = element.start;
      index.end = element.end;

      unpack.lhs.push_back(element.variable);
      unpack.rhs.push_back(std::make_shared<Parser::ExpressionValue>(std::move(index)));
    }

    scope().tableUnpacks.push_back(std::move(unpack));
  }

  void LuauTranspiler::emitPosition(const Parser::NodeBase& node) {
    if (!showDebugComments) {
      return;
    }

    out << " --[[ " << node.start.line << ":" << node.start.column << " to " << node.end.line << ":" << node.end.column
      << " ]]";
  }

  void LuauTranspiler::emitRegularOperator(const Parser::BinaryOperator& node, const std::string_view& op) {
    out << "(";
    emit(node.lhs);
    out << ") " << op << " (";
    emit(node.rhs);
    out << ")";
  }

  void LuauTranspiler::emitFunctionOperator(const Parser::BinaryOperator& node, const std::string_view& function) {
    out << function << "(";
    emit(node.lhs);
    out << ", ";
    emit(node.rhs);
    out << ")";
  }

  void LuauTranspiler::emitPipeOperator(const Parser::BinaryOperator& node) {
    auto pipedCall = Parser::Call{};
    pipedCall.start = node.start;
    pipedCall.end = node.end;

    if (std::holds_alternative<Parser::Call>(*node.rhs)) {
      const auto& originalCall = std::get<Parser::Call>(*node.rhs);

      std::vector newArguments = { node.lhs };
      newArguments.insert(newArguments.end(), originalCall.arguments.cbegin(), originalCall.arguments.cend());

      pipedCall.lhs = originalCall.lhs;
      pipedCall.arguments = std::move(newArguments);
    } else {
      pipedCall.lhs = node.rhs;
      pipedCall.arguments = { node.lhs };
    }

    emit(pipedCall);
  }

  void LuauTranspiler::emitAssignmentOperator(const Parser::BinaryOperator& node) {
    if (scope().isInExpression) {
      const auto retSymbol = "ret_" + uuidGenerator.generate().toString({ .hyphenated = false });

      pushExpressionScope();
      out << "(function() \nlocal " << retSymbol << " = ";

      emit(node.rhs);
      out << ";\n";

      emit(node.lhs);
      out << " = " << retSymbol << ";\n";

      out << "return " << retSymbol << " end)()";
      popScope();
    } else {
      pushExpressionScope();

      emit(node.lhs);
      out << " = ";
      emit(node.rhs);

      popScope();
    }

    emitPosition(node);
  }

  void LuauTranspiler::emitIndexOperator(const Parser::BinaryOperator& node) {
    const auto isOptional = node.op == Operator::OptionalIndex || node.op == Operator::OptionalIndexExpression;

    if (isOptional) {
      out << "(function() local lhs = ";
    }

    if (std::holds_alternative<Parser::Symbol>(*node.lhs)) {
      emit(node.lhs);
    } else {
      out << "(";
      emit(node.lhs);
      out << ")";
    }

    if (isOptional) {
      out << "; if lhs == nil then return nil else return lhs";
    }

    switch (node.op) {
      case Operator::Index:
      case Operator::OptionalIndex:
        out << ".";
        emit(node.rhs);
        break;
      case Operator::IndexExpression:
      case Operator::OptionalIndexExpression:
        out << "[";
        emit(node.rhs);
        out << "]";
        break;
      default:
        throw std::runtime_error("Invalid operator for index");
    }

    if (isOptional) {
      out << " end end)()";
    }
  }

  void LuauTranspiler::emitCoalesceOperator(const Parser::BinaryOperator& node) {
    out << "(function() local lhs = (";
    emit(node.lhs);
    out << "); if lhs == nil then return (";
    emit(node.rhs);
    out << ") else return lhs end end)()";
  }

  void LuauTranspiler::emitBlock(
    const Parser::Block& node,
    const std::string_view& returnStatement,
    const std::string_view& returnStatementPost
  ) {
    const auto shouldReturn = scope().isInExpression;
    pushStatementScope();

    for (const auto& line : node.expressions) {
      if (std::holds_alternative<Parser::Expression>(line)) {
        if (&line == &node.expressions.back() && shouldReturn) {
          out << returnStatement << " ";
          pushExpressionScope();
        }

        emit(std::get<Parser::Expression>(line));

        if (&line == &node.expressions.back() && shouldReturn) {
          popScope();
          out << returnStatementPost;
        }
      } else {
        emit(std::get<Parser::Statement>(line));
      }

      out << "\n";
    }

    popScope();
  }

  void LuauTranspiler::emitTableUnpacks(const std::vector<Parser::Definition>& unpacks) {
    for (const auto& unpack : unpacks) {
      emit(unpack);

      if (&unpack != &unpacks.back()) {
        out << ";\n";
      }
    }
  }

  void LuauTranspiler::emitExports(const Parser::LValue& lValue) {
    std::visit(
      [this](const auto& n) {
        emitExports(n);
      },
      *lValue
    );
  }

  void LuauTranspiler::emitExports(const Parser::Symbol& symbol) {
    out << "\n" << scope().exportsSymbol << "." << symbol.name << " = " << symbol.name << "\n";
  }

  void LuauTranspiler::emitExports(const Parser::TableUnpack& tableUnpack) {
    for (const auto& element : tableUnpack.elements) {
      emitExports(element.variable);
    }
  }

  void LuauTranspiler::pushExpressionScope() {
    const auto& currentScope = scope();

    scopes.push(
      {
        .isInExpression = true,
        .isInLValue = false,
        .exportsSymbol = currentScope.exportsSymbol,
      }
    );
  }

  void LuauTranspiler::pushLValueScope() {
    const auto& currentScope = scope();

    scopes.push(
      {
        .isInExpression = false,
        .isInLValue = true,
        .exportsSymbol = currentScope.exportsSymbol,
      }
    );
  }

  void LuauTranspiler::pushStatementScope() {
    const auto& currentScope = scope();

    scopes.push(
      {
        .isInExpression = false,
        .isInLValue = false,
        .exportsSymbol = currentScope.exportsSymbol,
      }
    );
  }

  void LuauTranspiler::pushTopLevelScope(std::string_view exportsSymbol) {
    scopes.push(
      {
        .isInExpression = false,
        .isInLValue = false,
        .exportsSymbol = exportsSymbol,
      }
    );
  }

  void LuauTranspiler::popScope() {
    scopes.pop();
  }

  LuauTranspiler::Scope& LuauTranspiler::scope() {
    return scopes.top();
  }
}
