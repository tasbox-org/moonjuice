<p align="center">
  <img height="256" src="https://github.com/tasbox-org/moonjuice/blob/master/branding/logo.png?raw=true" />
</p>
<p align="center">
  <img alt="Discord" src="https://img.shields.io/discord/1448019135304040599?style=for-the-badge&logo=discord&logoColor=white&label=discord&labelColor=%235865F2&color=%23E0E3FF">
  &nbsp  
  <img alt="GitHub License" src="https://img.shields.io/github/license/tasbox-org/moonjuice?style=for-the-badge">
  &nbsp  
  <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/tasbox-org/moonjuice/build-test-release.yaml?style=for-the-badge">
</p>

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
