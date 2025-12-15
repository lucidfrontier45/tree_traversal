use std::iter::FusedIterator;

use crate::{functional::bbs_reach, node::TreeNode};

/// Depth-First traversal implementation.
pub struct DepthFirstTraversal<N> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<C, N> DepthFirstTraversal<N>
where
    C: Default + Copy + Ord + 'static,
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
        let state = bbs_reach(
            root_node,
            |n: &N| n.generate_child_nodes(),
            |_: &N| false,
            |_: &N| None,
            |_: &N| Some(C::default()),
        );
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
