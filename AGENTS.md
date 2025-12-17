# AGENTS.md

## Project Overview

This is a Rust library called `tree_traversal` (version 0.4.0) for finding the optimal leaf node in a tree structure using various traversal algorithms.

### Purpose
Implements several tree traversal algorithms to find the best (lowest cost) leaf node in a tree, useful for combinatorial optimization problems like the knapsack problem.

### Algorithms Implemented
- Breadth First Search (BFS)
- Depth First Search (DFS)
- Beam Search (BMS)
- Branch and Bound Search (BBS)
- Greedy Search (GDS)
- Priority First Search (PFS)

### APIs
- Functional API: Under `traversal::functional`, provides standalone functions with pure function arguments.
- OOP API: Struct-based iterators for traversal.

### Tech Stack
- Language: Rust (Edition 2024)
- No external dependencies
- Pure library crate

### Codebase Structure
- `src/lib.rs`: Main library entry, exposes modules.
- `src/node.rs`: TreeNode trait definition.
- `src/traversal.rs`: Main traversal module, re-exports algorithms.
- `src/utils.rs`: Utility functions (private).
- `src/traversal/`: Submodules for each algorithm (bfs.rs, dfs.rs, etc.).
- `src/traversal/functional/`: Functional versions of algorithms.
- `examples/`: Example usages, like knapsack problem implementations.

### Development
- Repository: https://github.com/lucidfrontier45/tree_traversal
- Author: Du Shiqiao
- License: See LICENSE file

## Style and Conventions

### Documentation
- `#![forbid(missing_docs)]`: All public items must have documentation comments.
- Use `///` for public items, `//!` for modules.
- Include parameters, returns, examples where appropriate.

### Linting
- Clippy is used with some allowances:
  - `clippy::non_ascii_literal`: Allowed for non-ASCII literals.
  - `clippy::module_name_repetitions`: Allowed for module name repetitions.

### Code Style
- Follows standard Rust conventions.
- Uses generics extensively for type safety.
- Traits like `TreeNode` for abstraction.
- Functional programming style in functional API.
- OOP style with iterators in traversal structs.

### Naming
- Modules: snake_case (e.g., `traversal`, `functional`).
- Functions/Methods: snake_case.
- Types/Structs: PascalCase.
- Constants: SCREAMING_SNAKE_CASE.

### Error Handling
- Uses `Result` and `Option` appropriately.
- Algorithms return `Option` or `Result` based on success.

### Testing
- Examples in `examples/` directory for demonstration.
- Likely uses `cargo test` for unit tests (though not visible in structure).

### Formatting
- Uses `rustfmt` for code formatting.

## Suggested Commands

### Building
- `cargo build`: Compile the library.
- `cargo build --release`: Compile optimized release build.

### Testing
- `cargo test`: Run all tests.
- `cargo test --doc`: Run documentation tests.
- `cargo test --example <name>`: Run specific example.

### Running Examples
- `cargo run --example bbs_knapsack_problem`: Run the BBS knapsack example.
- `cargo run --example bbs_knapsack_problem_oop`: Run the OOP version.

### Linting and Formatting
- `cargo clippy`: Run Clippy linter.
- `cargo fmt`: Format code with rustfmt.
- `cargo fmt --check`: Check formatting without changing files.

### Documentation
- `cargo doc`: Generate documentation.
- `cargo doc --open`: Generate and open documentation in browser.

### Other
- `cargo check`: Check code without building.
- `cargo clean`: Clean build artifacts.

### Windows Specific
Since the system is Windows, use PowerShell (pwsh) for terminal commands.
- `ls` equivalent: `dir` or `Get-ChildItem`
- `cd`: `cd` or `Set-Location`
- `grep`: `Select-String` or `findstr`
- `find`: `Get-ChildItem -Recurse`
- Git commands work the same: `git status`, `git add`, etc.

## Task Completion Checklist

When a coding task is completed, run the following to ensure quality:

1. **Format Code**: `cargo fmt` to format the code.
2. **Lint**: `cargo clippy` to check for linting issues.
3. **Test**: `cargo test` to run all tests and ensure nothing is broken.
4. **Build**: `cargo build` to verify compilation.
5. **Documentation**: `cargo doc` to update docs if public APIs changed.
6. **Examples**: Run relevant examples with `cargo run --example <name>` to verify functionality.

Ensure all public items have documentation, as `#![forbid(missing_docs)]` is enforced.

If adding new features, add corresponding tests and examples.