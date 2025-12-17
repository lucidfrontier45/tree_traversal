# Project Overview

This is a Rust library called `tree_traversal` (version 0.4.0) for finding the optimal leaf node in a tree structure using various traversal algorithms.

## Purpose
Implements several tree traversal algorithms to find the best (lowest cost) leaf node in a tree, useful for combinatorial optimization problems like the knapsack problem.

## Algorithms Implemented
- Breadth First Search (BFS)
- Depth First Search (DFS)
- Beam Search (BMS)
- Branch and Bound Search (BBS)
- Greedy Search (GDS)
- Priority First Search (PFS)

## APIs
- Functional API: Under `traversal::functional`, provides standalone functions with pure function arguments.
- OOP API: Struct-based iterators for traversal.

## Tech Stack
- Language: Rust (Edition 2024)
- No external dependencies
- Pure library crate

## Codebase Structure
- `src/lib.rs`: Main library entry, exposes modules.
- `src/node.rs`: TreeNode trait definition.
- `src/traversal.rs`: Main traversal module, re-exports algorithms.
- `src/utils.rs`: Utility functions (private).
- `src/traversal/`: Submodules for each algorithm (bfs.rs, dfs.rs, etc.).
- `src/traversal/functional/`: Functional versions of algorithms.
- `examples/`: Example usages, like knapsack problem implementations.

## Development
- Repository: https://github.com/lucidfrontier45/tree_traversal
- Author: Du Shiqiao
- License: See LICENSE file