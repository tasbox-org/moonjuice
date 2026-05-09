# MoonJuice

> [!WARNING]
> MoonJuice is not ready for usage outside TASBox,
> and both the grammar and these crates may have breaking changes made at any time

- Lua -> Moon
- Elixir -> Juice

MoonJuice is the custom scripting language built for TASBox.
These crates are a work-in-progress extraction of the language from the engine
(to allow its use in other projects, such as LSPs).

- `moonjuice-common` - Shared types and helpers across all crates
- `moonjuice-lexer` - Lexer/tokeniser producing rich tokens suitable for formatters and LSPs (incl. e.g. comments)
- `moonjuice-parser` - Parser and definition for MoonJuice's grammar
- `moonjuice-transpiler-luau` - Transpiler used by TASBox to run MoonJuice on top of Luau
  - You **must** enable at minimum the `LuauConst2` feature flag
