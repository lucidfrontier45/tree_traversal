# Style and Conventions

## Code Style

- **Naming**: Use snake_case for functions, variables, and modules. PascalCase for types and structs.
- **Documentation**: All public functions and modules must have documentation comments (`//!` for modules, `///` for items). The `#![forbid(missing_docs)]` lint enforces this.
- **Linting**: Allows `clippy::non_ascii_literal` and `clippy::module_name_repetitions`.
- **Formatting**: Use `cargo fmt` for consistent formatting.
- **Imports**: Use `use` statements at the top of files.
- **Generics**: Use descriptive generic names like `N` for Node, `C` for Cost, etc., with trait bounds.
- **Error Handling**: Functions return `Option` for results, with `None` if not found.
- **Testing**: Include unit tests in `#[cfg(test)]` modules within the same file.
- **Examples**: Use closures for functions like `successor_fn`, `cost_fn`, etc.
- **Performance**: Algorithms are designed for optimization, using bounded types from `num_traits`.

## Design Patterns

- **Functional Programming**: Heavy use of closures and iterators.
- **Tree Traversal**: Standard algorithms adapted for finding optimal leaves.
- **API Design**: Inspired by `pathfinding` crate, with start node, successor function, cost function, etc.