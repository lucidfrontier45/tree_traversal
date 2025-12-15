use std::iter::FusedIterator;

use crate::{
    functional::bms_reach,
    node::{Approximate, TreeNode},
};

/// Greedy traversal implementation.
pub struct GreedyTraversal<N> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<C, N> GreedyTraversal<N>
where
    C: Copy + Ord + 'static,
    N: TreeNode<Cost = C> + Approximate<Cost = C> + 'static,
{
    /// Creates a new `GreedyTraversal` instance that performs a greedy search starting from the given root node.
    ///
    /// Greedy search always chooses the locally optimal choice at each step, aiming for an approximate solution.
    ///
    /// # Parameters
    /// - `root_node`: The starting node for the traversal.
    ///
    /// # Returns
    /// A new `GreedyTraversal` iterator.
    pub fn new(root_node: N) -> Self {
        let state = bms_reach(
            root_node,
            |n: &N| n.generate_child_nodes(),
            |n: &N| n.cost_approx(),
            usize::MAX,
            1,
        );
        Self {
            state: Box::new(state),
        }
    }
}

impl<N> Iterator for GreedyTraversal<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next()
    }
}
