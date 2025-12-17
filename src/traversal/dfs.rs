use std::iter::FusedIterator;

use crate::node::TreeNode;

use super::functional::dfs_reach;

/// Depth-First traversal implementation.
pub struct DepthFirstTraversal<N> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<C, N> DepthFirstTraversal<N>
where
    C: Copy + Ord + 'static,
    N: TreeNode<Cost = C> + 'static,
{
    /// Creates a new `DepthFirstTraversal` instance that performs a depth-first search starting from the given root node.
    ///
    /// Depth-first search explores as far as possible along each branch before backtracking.
    ///
    /// # Parameters
    /// - `root_node`: The starting node for the traversal.
    ///
    /// # Returns
    /// A new `DepthFirstTraversal` iterator.
    pub fn new(root_node: N) -> Self {
        let state = dfs_reach(root_node, |n: &N| n.generate_child_nodes());
        Self {
            state: Box::new(state),
        }
    }
}

impl<N> Iterator for DepthFirstTraversal<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next()
    }
}
