# Project Overview

This is a Rust library called `tree_traversal` for finding the optimal leaf node in a tree structure. It implements several tree traversal algorithms: Breadth First Search (BFS), Depth First Search (DFS), Beam Search (BMS), Branch and Bound Search (BBS), and Greedy Search (GDS).

The library is designed for combinatorial optimization problems, such as the knapsack problem, where you need to find the best leaf in a decision tree.

Tech stack: Rust 2021 edition, with dependency on `num-traits` for bounded types.

Repository: https://github.com/lucidfrontier45/tree_traversal

Codebase structure:
- `src/lib.rs`: Main library file, exports modules.
- `src/bfs.rs`: BFS implementation.
- `src/dfs.rs`: DFS implementation.
- `src/bms.rs`: Beam Search implementation.
- `src/bbs.rs`: Branch and Bound Search implementation.
- `src/gds.rs`: Greedy Search implementation.
- `examples/`: Contains example usage, like knapsack problem with BBS.

The API is inspired by the `pathfinding` crate.