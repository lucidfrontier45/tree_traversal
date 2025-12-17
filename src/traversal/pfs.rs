use std::iter::FusedIterator;

use crate::node::{Priority, TreeNode};

use super::functional::pfs_reach;

/// Priority-First traversal implementation.
pub struct PriorityFirstTraversal<N> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<C, N> PriorityFirstTraversal<N>
where
    C: Copy + Ord + 'static,
    N: TreeNode<Cost = C> + Priority + 'static,
{
    /// Creates a new `PriorityFirstTraversal` instance that performs a priority-first search starting from the given root node.
    ///
    /// Priority-first search explores nodes based on their priority values, always selecting the highest priority node first.
    ///
    /// # Parameters
    /// - `root_node`: The starting node for the traversal.
    ///
    /// # Returns
    /// A new `PriorityFirstTraversal` iterator.
    pub fn new(root_node: N) -> Self {
        let state = pfs_reach(
            root_node,
            |n: &N| n.generate_child_nodes(),
            |n: &N| n.priority(),
        );
        Self {
            state: Box::new(state),
        }
    }
}

impl<N> Iterator for PriorityFirstTraversal<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next()
    }
}

impl<N> FusedIterator for PriorityFirstTraversal<N> {}
