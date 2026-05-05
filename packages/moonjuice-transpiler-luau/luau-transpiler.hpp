#pragma once

#include <sstream>
#include <stack>
#include "shared/helpers/uuidv7.hpp"
#include "shared/language/parser/nodes.hpp"

namespace MoonJuice::Transpiler {
  class LuauTranspiler {
  public:
    explicit LuauTranspiler(bool showDebugComments = false);

    [[nodiscard]] std::string transpile(const Parser::Block& block);

  private:
    struct Scope {
      bool isInExpression;
      bool isInLValue;

      std::string_view exportsSymbol;
      std::vector<Parser::Definition> tableUnpacks;
    };

    bool showDebugComments;

    UUIDv7Generator uuidGenerator;

    std::stringstream out;
    std::stack<Scope> scopes;

    void emit(const Parser::Block& node);

    void emit(const Parser::Expression& node);

    void emit(const Parser::Statement& node);

    void emit(const Parser::LValue& node);

    void emit(const Parser::Nil& node);

    void emit(const Parser::Boolean& node);

    void emit(const Parser::Integer& node);

    void emit(const Parser::Number& node);

    void emit(const Parser::String& node);

    void emit(const Parser::TableDefinition& node);

    void emit(const Parser::TableDefinitionElement& node);

    void emit(const Parser::Symbol& node);

    void emit(const Parser::UnaryOperator& node);

    void emit(const Parser::BinaryOperator& node);

    void emit(const Parser::Function& node);

    void emit(const Parser::If& node);

    void emit(const Parser::For& node);

    void emit(const Parser::Call& node);

    void emit(const Parser::Definition& node);

    void emit(const Parser::Return& node);

    void emit(const Parser::Break& node);

    void emit(const Parser::TableUnpack& node);

    void emitPosition(const Parser::NodeBase& node);

    template<typename TNode>
    void emitCommaSeparated(const std::vector<TNode>& nodes) {
      for (const auto& node : nodes) {
        emit(node);

        if (&node != &nodes.back()) {
          out << ", ";
        }
      }
    }

    void emitRegularOperator(const Parser::BinaryOperator& node, const std::string_view& op);

    void emitFunctionOperator(const Parser::BinaryOperator& node, const std::string_view& function);

    void emitPipeOperator(const Parser::BinaryOperator& node);

    void emitAssignmentOperator(const Parser::BinaryOperator& node);

    void emitIndexOperator(const Parser::BinaryOperator& node);

    void emitCoalesceOperator(const Parser::BinaryOperator& node);

    void emitBlock(
      const Parser::Block& node,
      const std::string_view& returnStatement,
      const std::string_view& returnStatementPost = ""
    );

    void emitTableUnpacks(const std::vector<Parser::Definition>& unpacks);

    void emitExports(const Parser::LValue& lValue);

    void emitExports(const Parser::Symbol& symbol);

    void emitExports(const Parser::TableUnpack& tableUnpack);

    void pushExpressionScope();

    void pushLValueScope();

    void pushStatementScope();

    void pushTopLevelScope(std::string_view exportsSymbol);

    void popScope();

    [[nodiscard]] Scope& scope();
  };
}
