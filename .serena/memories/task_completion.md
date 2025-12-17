# Task Completion Checklist

When a coding task is completed, run the following to ensure quality:

1. **Format Code**: `cargo fmt` to format the code.
2. **Lint**: `cargo clippy` to check for linting issues.
3. **Test**: `cargo test` to run all tests and ensure nothing is broken.
4. **Build**: `cargo build` to verify compilation.
5. **Documentation**: `cargo doc` to update docs if public APIs changed.
6. **Examples**: Run relevant examples with `cargo run --example <name>` to verify functionality.

Ensure all public items have documentation, as `#![forbid(missing_docs)]` is enforced.

If adding new features, add corresponding tests and examples.