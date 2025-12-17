# Tree Traversal Algorithms

This document explains the search algorithms implemented in the `tree_traversal` library for finding the optimal leaf node in a tree structure.

## Overview

The library implements six different traversal algorithms to solve combinatorial optimization problems, particularly useful for problems like the knapsack problem where we need to find the best solution among many possibilities. Each algorithm uses a different strategy to explore the solution space efficiently.

## Algorithms

### Breadth First Search (BFS)

**Description**: Breadth-first search explores the tree level by level, visiting all nodes at the current depth before moving to the next level.

**How it works**:
- Uses a queue (FIFO) to store nodes for exploration
- Starts with the root node in the queue
- At each step, removes the front node from the queue and adds its children to the back
- Explores nodes in order of increasing depth

**Key Characteristics**:
- Guarantees finding the optimal solution if the tree is finite and costs are non-decreasing
- Memory usage can be high for wide trees
- Systematic exploration of the search space

**Use Cases**:
- When you need guaranteed optimality and have sufficient memory
- For problems where solutions at shallow depths are preferred
- When the branching factor is not too high

**Implementation**:
- Uses `BreadthFirstContainer` with a `VecDeque` for FIFO ordering
- Functional API: `bfs_reach()` for traversal, `bfs()` for optimization

### Depth First Search (DFS)

**Description**: Depth-first search explores as far as possible along each branch before backtracking.

**How it works**:
- Uses a stack (LIFO) to store nodes for exploration
- Starts with the root node on the stack
- At each step, removes the top node from the stack and adds its children to the top
- Explores nodes by going deep into the tree

**Key Characteristics**:
- Low memory usage compared to BFS
- May find a solution quickly if it's deep in the tree
- Does not guarantee optimality unless the problem has special properties
- Can get stuck in deep branches

**Use Cases**:
- When memory is limited
- For problems where solutions are likely to be deep in the tree
- When you want to explore one path completely before others

**Implementation**:
- Uses `DepthFirstContainer` with a `Vec` as a stack
- Functional API: `dfs_reach()` for traversal, `dfs()` for optimization

### Beam Search (BMS)

**Description**: Beam search is a heuristic search algorithm that maintains a fixed-size "beam" of the most promising nodes at each level to limit memory usage.

**How it works**:
- Uses a priority queue to select the best nodes
- At each depth level, generates successors from current beam nodes
- Evaluates successors using a priority function
- Keeps only the top `beam_width` nodes for the next iteration
- Limits branching factor to `branch_factor` per node

**Key Characteristics**:
- Balances between optimality and computational efficiency
- Memory usage is bounded by beam width
- Quality depends on the priority function and beam parameters
- Not guaranteed to find the optimal solution

**Use Cases**:
- Large search spaces where full exploration is impossible
- When you need approximate solutions quickly
- Problems with good heuristic functions for node evaluation

**Parameters**:
- `branch_factor`: Maximum successors per node (default unlimited)
- `beam_width`: Maximum nodes kept per level

**Implementation**:
- Uses `BeamContainer` with `BinaryHeap` and `VecDeque`
- Functional API: `bms_reach()` for traversal, `bms()` for optimization
- Requires nodes to implement `Priority` trait

### Branch and Bound Search (BBS)

**Description**: Branch and bound is an algorithm for finding optimal solutions by systematically enumerating candidate solutions, using bounds to prune branches that cannot produce better solutions than the current best.

**How it works**:
- Explores the tree depth-first but with pruning
- For each node, computes a lower bound on the cost of any solution in its subtree
- Prunes branches where the lower bound is worse than the current best solution
- Updates the best solution when leaf nodes are found

**Key Characteristics**:
- Guarantees optimality for minimization problems
- Can prune large parts of the search space
- Efficiency depends on the quality of lower bounds
- May still explore the entire tree if bounds are poor

**Use Cases**:
- Combinatorial optimization problems with good lower bounds
- When you need guaranteed optimal solutions
- Problems where partial solutions can be bounded effectively

**Implementation**:
- Uses `BranchAndBoundContainer` with stack-based exploration
- Functional API: `bbs_reach()` for traversal, `bbs()` for optimization
- Requires nodes to implement `LowerBound` trait

### Greedy Search (GDS)

**Description**: Greedy search always chooses the locally optimal choice at each step, aiming for an approximate solution.

**How it works**:
- At each step, selects the best immediate successor based on a priority function
- Does not backtrack or explore alternative paths
- Continues until a leaf node is reached

**Key Characteristics**:
- Very fast and memory-efficient
- May find good solutions quickly
- Does not guarantee optimality
- Performance depends heavily on the priority function

**Use Cases**:
- When you need quick approximate solutions
- Problems with strong local heuristics
- As a baseline for comparison with other algorithms

**Implementation**:
- Uses `GreedyContainer` that keeps only one node at a time
- Functional API: `gds_reach()` for traversal, `gds()` for optimization
- Requires nodes to implement `Priority` trait

### Priority First Search (PFS)

**Description**: Priority-first search explores nodes based on their priority values, always selecting the highest priority node first.

**How it works**:
- Uses a priority queue to always select the best node
- All nodes are stored in the queue with their priorities
- At each step, pops the highest priority node and expands it
- Continues until the queue is empty or limits are reached

**Key Characteristics**:
- Systematic exploration based on priority
- Memory usage depends on queue size
- Order depends on priority function
- Can implement various search strategies (best-first, Dijkstra, A*)

**Use Cases**:
- When you have a good priority function for guiding search
- Problems where node evaluation is computationally cheap
- When you want controlled exploration order

**Implementation**:
- Uses `PriorityFirstContainer` with `BinaryHeap`
- Functional API: `pfs_reach()` for traversal, `pfs()` for optimization
- Requires nodes to implement `Priority` trait

## Common Concepts

### TreeNode Trait
All algorithms work with nodes that implement the `TreeNode` trait:
- `generate_child_nodes()`: Returns child nodes
- `cost()`: Returns the cost of the node
- `is_leaf()`: Checks if the node is a leaf

### Additional Traits
Some algorithms require additional traits:
- `Priority`: For algorithms that need node evaluation (BMS, GDS, PFS)
- `LowerBound`: For branch and bound pruning

### Functional vs OOP API
- **Functional API**: Pure functions that take closures for successor generation and evaluation
- **OOP API**: Iterator-based structs that encapsulate the traversal logic

### Optimization Functions
Each algorithm provides an optimization function (e.g., `bfs()`, `dfs()`) that:
- Traverses the tree
- Finds the best (lowest cost) leaf node
- Respects operation and time limits
- Returns `Some((cost, node))` or `None` if no solution found

## Choosing an Algorithm

- **Use BFS** when you need guaranteed optimality and have memory for wide trees
- **Use DFS** when memory is limited and you suspect solutions are deep
- **Use BBS** for optimization problems with good lower bounds
- **Use BMS** for large spaces needing approximation with bounded memory
- **Use GDS** for quick approximations
- **Use PFS** when you have a reliable priority function

The choice depends on problem characteristics, available memory, time constraints, and whether optimality is required.