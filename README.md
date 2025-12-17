# tree_traversal
A Rust library for finding the optimal leaf node in a tree structure.
This crate implements several tree traversal algorithms to find the best (the lowest cost) leaf node in a tree.

# Algorithms

- Breadth First Search
- Depth First Search
- Beam Search
- Branch and Bound Search
- Greedy Search

# Using this crate

```bash
cargo add tree_traversal
```



# APIs

## Functional API

The crate exposes a functional-style API under `traversal::functional` which provides traversal
algorithms as standalone functions you can call with pure function arguments (successor, cost,
leaf-check, etc.). This style is lightweight and works well when your problem can be expressed
with small functions or closures. The example above demonstrates calling `traversal::functional::bbs`:

- **Where**: use `traversal::functional`.
- **What**: functions like `bbs`, `bfs`, `dfs`, `bms`, and `gds`.
- **Inputs**: initial state, `successor_fn`, `leaf_check_fn`, `cost_fn`, bounds/limits.
- **Outputs**: `(cost, best_node)` or `Option`/`Result` depending on the algorithm.

Use this API when you prefer functional composition, want minimal boilerplate, or are using
small/anonymous state representations (tuples, `Vec`, simple structs).

#### Functional Example

The following demonstrates using the functional `bbs` function from `traversal::functional`.

```rust
use tree_traversal::traversal::functional::bbs;

type Node = Vec<bool>;
let weights = [4, 2, 6, 3, 4];
let profits = [100, 20, 2, 5, 10];
let capacity = 8 as u32;
let total_items = weights.len();

let successor_fn = |n: &Node| {
    if n.len() == total_items {
        return vec![];
    }

    let total_weight: u32 = n
        .iter()
        .copied()
        .enumerate()
        .map(|(i, b)| if b { weights[i] } else { 0 })
        .sum();

    let mut children = vec![];

    let next_idx = n.len();
    if capacity >= total_weight + weights[next_idx] {
        let mut c1 = n.clone();
        c1.push(true);
        children.push(c1);
    }

    let mut c2 = n.clone();
    c2.push(false);
    children.push(c2);

    children
};

let total_profit = |n: &Node| {
    n.iter()
        .copied()
        .enumerate()
        .map(|(i, b)| if b { profits[i] } else { 0 })
        .sum::<u32>()
};

let lower_bound_fn = |n: &Node| {
    let current_profit = total_profit(n);
    let max_remained_profit: u32 = profits[n.len()..].into_iter().sum();
    Some(u32::MAX - (current_profit + max_remained_profit))
};

let cost_fn = |n: &Node| Some(u32::MAX - total_profit(n));

let leaf_check_fn = |n: &Node| n.len() == total_items;
let max_ops = usize::MAX;
let time_limit = std::time::Duration::from_secs(10);

let (cost, best_node) = bbs(
    vec![],
    successor_fn,
    leaf_check_fn,
    cost_fn,
    lower_bound_fn,
    max_ops,
    time_limit,
)
.unwrap();
let cost = u32::MAX - cost;

dbg!((best_node, cost));
```

## OOP API

The crate also provides a more object-oriented API which groups traversal behavior together
with state in structs and traits. This style is useful when you have complex state, want to
encapsulate helper methods, or want to implement custom strategies by implementing traits.

- **Where**: top-level `traversal` modules and structs (see source for concrete types).
- **What**: traversal types that can own state and expose methods to run searches.
- **Inputs**: implement or instantiate the provided traits/structs, then call the method on the traversal object to execute the search.

Use the OOP API when you prefer encapsulation, need to share rich state between helpers, or
when building reusable components around a traversal (for example, packing problem solvers
or search strategies that maintain internal caches or statistics).

If you're not sure which to pick, start with the Functional API for quick experiments and move
to the OOP API when your state or logic grows in complexity.

#### OOP Example

Below is a minimal example showing how to implement the `TreeNode` trait for a simple
knapsack example from the `examples` directory and shows a more complete, idiomatic use of
the OOP API (uses `LowerBound`, `TreeNode`, and `BranchAndBoundTraversal`).

```rust
use std::rc::Rc;

use tree_traversal::{
    node::{LowerBound, TreeNode},
    traversal::BranchAndBoundTraversal,
};

struct Node {
    items: Vec<bool>,
    capacity: u32,
    weights: Rc<[u32]>,
    profits: Rc<[u32]>,
}

impl Node {
    fn total_profit(&self) -> u32 {
        self.items
            .iter()
            .copied()
            .enumerate()
            .map(|(i, b)| if b { self.profits[i] } else { 0 })
            .sum()
    }

    fn max_profit(&self) -> u32 {
        let current_profit = self.total_profit();
        let max_remained_profit: u32 = self.profits[self.items.len()..].iter().sum();
        current_profit + max_remained_profit
    }
}

impl LowerBound for Node {
    type Cost = u32;

    fn cost_lb(&self) -> Option<Self::Cost> {
        let max_profit = self.max_profit();
        Some(u32::MAX - max_profit)
    }
}

impl TreeNode for Node {
    type Cost = u32;

    fn is_leaf(&self) -> bool {
        self.profits.len() == self.items.len()
    }

    fn generate_child_nodes(&self) -> Vec<Self> {
        if self.is_leaf() {
            return vec![];
        }

        let total_weight: u32 = self
            .items
            .iter()
            .copied()
            .enumerate()
            .map(|(i, b)| if b { self.weights[i] } else { 0 })
            .sum();

        let mut children = vec![];

        let next_idx = self.items.len();
        if self.capacity >= total_weight + self.weights[next_idx] {
            let mut c1 = self.items.clone();
            c1.push(true);
            children.push(Node {
                items: c1,
                capacity: self.capacity,
                weights: self.weights.clone(),
                profits: self.profits.clone(),
            });
        }

        let mut c2 = self.items.clone();
        c2.push(false);
        children.push(Node {
            items: c2,
            capacity: self.capacity,
            weights: self.weights.clone(),
            profits: self.profits.clone(),
        });

        children
    }

    fn cost(&self) -> Option<Self::Cost> {
        let profit = self.total_profit();
        Some(u32::MAX - profit)
    }
}

let weights = [4, 2, 6, 3, 4];
let profits = [100, 20, 2, 5, 10];
let capacity = 8;

let root_node = Node {
    items: vec![],
    capacity,
    weights: Rc::new(weights),
    profits: Rc::new(profits),
};

let mut traversal = BranchAndBoundTraversal::new(root_node);
let result = tree_traversal::traversal::find_best(
    &mut traversal,
    10000,
    std::time::Duration::from_secs(100),
    |_, _| {},
);
if let Some((cost, node)) = result {
    println!("Best profit: {}", u32::MAX - cost);
    println!("Items taken: {:?}", node.items);
} else {
    println!("No solution found");
}
```

# Note

The Functional API is derived from the great [pathfinding](https://docs.rs/pathfinding/latest/pathfinding/index.html) crate.
