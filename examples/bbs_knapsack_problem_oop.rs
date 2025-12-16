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

fn main() {
    let weights = [4, 2, 6, 3, 4];
    let profits = [100, 20, 2, 5, 10];
    let capacity = 8;

    let root_node = Node {
        items: vec![],
        capacity,
        weights: Rc::new(weights),
        profits: Rc::new(profits),
    };

    let null_callback = |_: usize, _: &Node| {};

    let mut traversal = BranchAndBoundTraversal::new(root_node).fuse();
    let result = tree_traversal::traversal::find_best(
        &mut traversal,
        10000,
        std::time::Duration::from_secs(100),
        null_callback,
    );
    if let Some((cost, node)) = result {
        println!("Best profit: {}", u32::MAX - cost);
        println!("Items taken: {:?}", node.items);
    } else {
        println!("No solution found");
    }
}
