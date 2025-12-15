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

/// Traverses the tree using the provided traversal iterator, collecting the best leaf nodes.
///
/// This function wraps the functional `traverse` with default leaf and cost functions based on the `TreeNode` trait.
///
/// # Parameters
/// - `traversal`: A mutable reference to a traversal iterator.
/// - `max_ops`: The maximum number of nodes to process.
/// - `time_limit`: The maximum time allowed for the traversal.
/// - `queue_size`: The maximum number of best nodes to return.
///
/// # Returns
/// A vector of tuples containing the cost and the node, sorted by cost (lowest first).
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

/// Finds the best leaf node in the tree using the provided traversal iterator.
///
/// This function wraps `traverse` to return only the single best node.
///
/// # Parameters
/// - `traversal`: A mutable reference to a traversal iterator.
/// - `max_ops`: The maximum number of nodes to process.
/// - `time_limit`: The maximum time allowed for the traversal.
///
/// # Returns
/// The best leaf node and its cost, or `None` if no leaf is found.
pub fn find_best<N: TreeNode>(
    traversal: &mut impl FusedIterator<Item = N>,
    max_ops: usize,
    time_limit: Duration,
) -> Option<(<N as TreeNode>::Cost, N)> {
    traverse(traversal, max_ops, time_limit, 1).pop()
}
