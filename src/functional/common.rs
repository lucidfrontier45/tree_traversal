//! Common functional utilities for tree traversal algorithms.

use std::{cmp::Reverse, collections::BinaryHeap, iter::FusedIterator, time::Duration};

use crate::utils::ScoredItem;

/// Generic search function over a tree represented by a fused iterator.
pub fn traverse<C, N, FC, FL>(
    tree: &mut impl FusedIterator<Item = N>,
    leaf_check_fn: FL,
    cost_fn: FC,
    max_ops: usize,
    time_limit: Duration,
    queue_size: usize,
) -> Vec<(C, N)>
where
    C: Ord + Copy,
    FC: Fn(&N) -> Option<C>,
    FL: Fn(&N) -> bool,
{
    let mut queue = BinaryHeap::new();

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

        queue.push(ScoredItem::from((Reverse(cost), n)));
    }

    queue
        .into_iter()
        .take(queue_size)
        .map(|item| {
            let (Reverse(cost), n) = item.into_inner();
            (cost, n)
        })
        .collect()
}

/// Generic search function over a tree represented by a fused iterator.
pub fn find_best<C, N, FC, FL>(
    tree: &mut impl FusedIterator<Item = N>,
    leaf_check_fn: FL,
    cost_fn: FC,
    max_ops: usize,
    time_limit: Duration,
) -> Option<(C, N)>
where
    C: Ord + Copy,
    FC: Fn(&N) -> Option<C>,
    FL: Fn(&N) -> bool,
{
    traverse(
        tree,
        leaf_check_fn,
        cost_fn,
        max_ops,
        time_limit,
        1, // only need the best one
    )
    .pop()
}
