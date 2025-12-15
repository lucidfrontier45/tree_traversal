//! Common functional utilities for tree traversal algorithms.

use std::{iter::FusedIterator, time::Duration};

/// Generic search function over a tree represented by a fused iterator.
pub fn search<C, N, FC, FL>(
    tree: &mut impl FusedIterator<Item = N>,
    leaf_check_fn: FL,
    cost_fn: FC,
    max_ops: usize,
    time_limit: Duration,
) -> Option<(C, N)>
where
    C: Ord + Copy,
    N: Clone,
    FC: Fn(&N) -> Option<C>,
    FL: Fn(&N) -> bool,
{
    let mut current_best_node = None;
    let mut current_best_cost = None;

    let start = std::time::Instant::now();
    for (i, n) in tree.enumerate() {
        if i >= max_ops || start.elapsed() >= time_limit {
            break;
        }

        if !leaf_check_fn(&n) {
            continue;
        }

        let Some(cost) = cost_fn(&n) else {
            continue;
        };

        if let Some(best_cost) = current_best_cost
            && cost >= best_cost
        {
            continue;
        }

        current_best_node = Some(n);
        current_best_cost = Some(cost);
    }

    current_best_node.map(|n| (current_best_cost.unwrap(), n))
}
