# Suggested Commands

## Building
- `cargo build`: Compile the library.
- `cargo build --release`: Compile optimized release build.

## Testing
- `cargo test`: Run all tests.
- `cargo test --doc`: Run documentation tests.
- `cargo test --example <name>`: Run specific example.

## Running Examples
- `cargo run --example bbs_knapsack_problem`: Run the BBS knapsack example.
- `cargo run --example bbs_knapsack_problem_oop`: Run the OOP version.

## Linting and Formatting
- `cargo clippy`: Run Clippy linter.
- `cargo fmt`: Format code with rustfmt.
- `cargo fmt --check`: Check formatting without changing files.

## Documentation
- `cargo doc`: Generate documentation.
- `cargo doc --open`: Generate and open documentation in browser.

## Other
- `cargo check`: Check code without building.
- `cargo clean`: Clean build artifacts.

## Windows Specific
Since the system is Windows, use PowerShell (pwsh) for terminal commands.
- `ls` equivalent: `dir` or `Get-ChildItem`
- `cd`: `cd` or `Set-Location`
- `grep`: `Select-String` or `findstr`
- `find`: `Get-ChildItem -Recurse`
- Git commands work the same: `git status`, `git add`, etc.