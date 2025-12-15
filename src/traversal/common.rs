use std::{iter::FusedIterator, time::Duration};

use crate::{functional, node::TreeNode};

/// Trait defining the interface for tree traversal algorithms.
pub trait Traversal<N: TreeNode>: FusedIterator<Item = N> {}

impl<T, N> Traversal<N> for T
where
    T: FusedIterator<Item = N> + ?Sized,
    N: TreeNode,
{
}

/// Traverses the tree up to a maximum number of operations or until the optional
/// `time_limit` has elapsed. Returns all found leaves (cost and node).
pub fn traverse<N: TreeNode>(
    traversal: &mut impl FusedIterator<Item = N>,
    max_ops: usize,
    time_limit: Duration,
    queue_size: usize,
) -> Vec<(<N as TreeNode>::Cost, N)> {
    functional::traverse(
        traversal,
        |n: &N| n.is_leaf(),
        |n: &N| n.cost(),
        max_ops,
        time_limit,
        queue_size,
    )
}

/// Traverses the tree up to a maximum number of operations or until the optional
/// `time_limit` has elapsed. Returns the best leaf found (cost and node), if any.
pub fn find_best<N: TreeNode>(
    traversal: &mut impl FusedIterator<Item = N>,
    max_ops: usize,
    time_limit: Duration,
) -> Option<(<N as TreeNode>::Cost, N)> {
    traverse(traversal, max_ops, time_limit, 1).pop()
}
