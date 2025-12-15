use std::iter::FusedIterator;

use crate::node::{LowerBound, TreeNode};

use super::functional::bbs_reach;

/// Branch-and-Bound traversal implementation.
pub struct BranchAndBoundTraversal<N> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<N, C> BranchAndBoundTraversal<N>
where
    C: Copy + Ord + 'static,
    N: TreeNode<Cost = C> + LowerBound<Cost = C> + 'static,
{
    /// Creates a new `BranchAndBoundTraversal` instance that performs a branch-and-bound search starting from the given root node.
    ///
    /// Branch-and-bound is an algorithm for finding optimal solutions by systematically enumerating candidate solutions,
    /// using bounds to prune branches that cannot produce better solutions than the current best.
    ///
    /// # Parameters
    /// - `root_node`: The starting node for the traversal.
    ///
    /// # Returns
    /// A new `BranchAndBoundTraversal` iterator.
    pub fn new(root_node: N) -> Self {
        let state = bbs_reach(
            root_node,
            |n: &N| n.generate_child_nodes(),
            |n: &N| n.is_leaf(),
            |n: &N| n.cost(),
            |n: &N| n.cost_lb(),
        );
        Self {
            state: Box::new(state),
        }
    }
}

impl<N> Iterator for BranchAndBoundTraversal<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next()
    }
}
