use std::{iter::FusedIterator, time::Duration};

use crate::{functional::common::search, node::TreeNode};

/// Trait defining the interface for tree traversal algorithms.
pub trait Traversal<N: TreeNode>: FusedIterator<Item = N> {}

impl<T, N> Traversal<N> for T
where
    T: FusedIterator<Item = N> + ?Sized,
    N: TreeNode,
{
}

/// Traverses the tree up to a maximum number of operations or until the optional
/// `time_limit` has elapsed. Returns the best leaf found (cost and node), if any.
pub fn traverse<N: TreeNode>(
    traversal: &mut impl FusedIterator<Item = N>,
    max_ops: usize,
    time_limit: Duration,
) -> Option<(<N as TreeNode>::Cost, N)> {
    search(
        traversal,
        |n: &N| n.is_leaf(),
        |n: &N| n.cost(),
        max_ops,
        time_limit,
    )
}
