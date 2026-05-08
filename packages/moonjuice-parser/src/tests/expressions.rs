#[cfg(test)]
mod tests {
  use crate::snapshot;

  snapshot!(should_parse_call, "a()");
  snapshot!(should_parse_call_with_args, "a(1, 2)");
  snapshot!(should_parse_optional_call, "a?.()");

  snapshot!(should_parse_index, "a.b");
  snapshot!(should_parse_chained_index, "a.b.c");
  snapshot!(should_parse_optional_index, "a?.b");
  snapshot!(should_parse_index_expression, "a[1]");
  snapshot!(should_parse_optional_index_expression, "a?.[1]");

  snapshot!(should_parse_block, "do end");
  snapshot!(should_parse_block_with_body, "do 1 end");

  snapshot!(should_parse_function_definition, "fn() end");
  snapshot!(should_parse_function_definition_with_body, "fn() 1 end");
  snapshot!(should_parse_function_definition_with_parameters, "fn(a, b) end");
  snapshot!(
    should_parse_function_definition_with_table_unpack_parameter,
    "fn({ .a, .b }) end"
  );

  snapshot!(should_parse_for_loop, "for i in a do end");
  snapshot!(should_parse_for_loop_with_multiple_return, "for k, v in a do end");
  snapshot!(should_parse_for_loop_with_body, "for i in a do 1 end");
  snapshot!(
    should_parse_for_loop_with_table_unpack_iterator,
    "for { a, b } in a do end"
  );

  snapshot!(should_parse_if, "if true then end");
  snapshot!(
    should_parse_if_elseif_chain,
    "if 1 then elseif 2 then elseif 3 then end"
  );
  snapshot!(should_parse_if_else_chain, "if 1 then else end");
  snapshot!(
    should_parse_if_elseif_else_chain,
    "if 1 then elseif 2 then elseif 3 then else end"
  );

  snapshot!(should_parse_whole_string, "'contents'");
  snapshot!(should_parse_format_string, "'{1}'");
  snapshot!(
    should_parse_format_string_with_multiple_parts,
    "'first{1}second{2}third{3}last'"
  );
}
