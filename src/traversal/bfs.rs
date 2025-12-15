use std::iter::FusedIterator;

use crate::{functional::bms_reach, node::TreeNode};

/// Breadth-First traversal implementation.
pub struct BreadthFirstTraversal<N> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<C, N> BreadthFirstTraversal<N>
where
    C: Default + Copy + Ord + 'static,
    N: TreeNode<Cost = C> + 'static,
{
    /// Creates a new `BreadthFirstTraversal` instance that performs a breadth-first search starting from the given root node.
    ///
    /// Breadth-first search explores nodes level by level, visiting all nodes at the current depth before moving to the next.
    ///
    /// # Parameters
    /// - `root_node`: The starting node for the traversal.
    ///
    /// # Returns
    /// A new `BreadthFirstTraversal` iterator.
    pub fn new(root_node: N) -> Self {
        let state = bms_reach(
            root_node,
            |n: &N| n.generate_child_nodes(),
            |_: &N| Some(C::default()),
            usize::MAX,
            usize::MAX,
        );
        Self {
            state: Box::new(state),
        }
    }
}

impl<N> Iterator for BreadthFirstTraversal<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next()
    }
}
