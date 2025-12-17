# Style and Conventions

## Documentation
- `#![forbid(missing_docs)]`: All public items must have documentation comments.
- Use `///` for public items, `//!` for modules.
- Include parameters, returns, examples where appropriate.

## Linting
- Clippy is used with some allowances:
  - `clippy::non_ascii_literal`: Allowed for non-ASCII literals.
  - `clippy::module_name_repetitions`: Allowed for module name repetitions.

## Code Style
- Follows standard Rust conventions.
- Uses generics extensively for type safety.
- Traits like `TreeNode` for abstraction.
- Functional programming style in functional API.
- OOP style with iterators in traversal structs.

## Naming
- Modules: snake_case (e.g., `traversal`, `functional`).
- Functions/Methods: snake_case.
- Types/Structs: PascalCase.
- Constants: SCREAMING_SNAKE_CASE.

## Error Handling
- Uses `Result` and `Option` appropriately.
- Algorithms return `Option` or `Result` based on success.

## Testing
- Examples in `examples/` directory for demonstration.
- Likely uses `cargo test` for unit tests (though not visible in structure).

## Formatting
- Uses `rustfmt` for code formatting.