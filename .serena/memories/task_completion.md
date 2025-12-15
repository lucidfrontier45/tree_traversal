# Task Completion Checklist

When a task is completed (e.g., adding a new feature, fixing a bug, refactoring code):

1. **Run Tests**: Execute `cargo test` to ensure all tests pass.
2. **Format Code**: Run `cargo fmt` to format the code according to Rust standards.
3. **Lint Code**: Run `cargo clippy` to check for potential issues and improvements.
4. **Build Project**: Run `cargo build` to ensure the project compiles without errors.
5. **Update Documentation**: If public API changed, update docs and run `cargo doc`.
6. **Commit Changes**: Use git to commit the changes with a descriptive message.
7. **Push to Remote**: Push the changes to the repository.

This ensures code quality, correctness, and maintainability.