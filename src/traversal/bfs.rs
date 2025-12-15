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
    /// Creates a new BreadthFirstTraversal starting from the given root node.
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
