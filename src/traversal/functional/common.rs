//! Common functional utilities for tree traversal algorithms.

use std::{collections::BinaryHeap, iter::FusedIterator, time::Duration};

use crate::utils::ScoredItem;

/// Performs a generic traversal over a tree iterator, collecting the best leaf nodes based on their costs.
///
/// This function iterates through the provided tree, checks each node to determine if it's a leaf using the `leaf_check_fn`,
/// computes its cost using the `cost_fn`, and maintains a priority queue of the top `queue_size` nodes with the lowest costs.
/// The traversal stops early if the maximum number of operations (`max_ops`) is reached or the time limit is exceeded.
///
/// # Parameters
/// - `tree`: A mutable reference to a fused iterator over the tree nodes.
/// - `leaf_check_fn`: A function that checks if a node is a leaf.
/// - `cost_fn`: A function that computes the cost of a node, returning `None` if the cost cannot be determined.
/// - `max_ops`: The maximum number of nodes to process.
/// - `time_limit`: The maximum time allowed for the traversal.
/// - `queue_size`: The maximum number of best nodes to keep in the result.
///
/// # Returns
/// A vector of tuples containing the cost and the node, limited to `queue_size`.
pub fn traverse<C, N, FC, FL, CB>(
    tree: &mut impl FusedIterator<Item = N>,
    leaf_check_fn: FL,
    cost_fn: FC,
    max_ops: usize,
    time_limit: Duration,
    queue_size: usize,
    mut callback_fn: CB,
) -> Vec<(C, N)>
where
    C: Ord + Copy,
    FC: Fn(&N) -> Option<C>,
    FL: Fn(&N) -> bool,
    CB: FnMut(usize, &N),
{
    let mut queue = BinaryHeap::new();

    let start = std::time::Instant::now();
    for (i, n) in tree.enumerate() {
        if i >= max_ops || start.elapsed() >= time_limit {
            break;
        }
        callback_fn(i, &n);

        if !leaf_check_fn(&n) {
            continue;
        }

        let Some(cost) = cost_fn(&n) else {
            continue;
        };

        queue.push(ScoredItem::from((cost, n)));
        if queue.len() > queue_size {
            queue.pop();
        }
    }

    queue
        .into_iter()
        .take(queue_size)
        .map(|item| {
            let (cost, n) = item.into_inner();
            (cost, n)
        })
        .collect()
}

/// Finds the best (lowest cost) leaf node in the tree iterator within the given constraints.
///
/// This function is a convenience wrapper around `traverse` that returns only the single best node.
///
/// # Parameters
/// - `tree`: A mutable reference to a fused iterator over the tree nodes.
/// - `leaf_check_fn`: A function that checks if a node is a leaf.
/// - `cost_fn`: A function that computes the cost of a node, returning `None` if the cost cannot be determined.
/// - `max_ops`: The maximum number of nodes to process.
/// - `time_limit`: The maximum time allowed for the traversal.
///
/// # Returns
/// The best (lowest cost) leaf node and its cost, or `None` if no valid leaf is found.
pub fn find_best<C, N, FC, FL, CB>(
    tree: &mut impl FusedIterator<Item = N>,
    leaf_check_fn: FL,
    cost_fn: FC,
    max_ops: usize,
    time_limit: Duration,
    mut callback_fn: CB,
) -> Option<(C, N)>
where
    C: Ord + Copy,
    FC: Fn(&N) -> Option<C>,
    FL: Fn(&N) -> bool,
    CB: FnMut(usize, &N),
{
    traverse(
        tree,
        leaf_check_fn,
        cost_fn,
        max_ops,
        time_limit,
        1, // only need the best one
        &mut callback_fn,
    )
    .pop()
}
