use std::iter::FusedIterator;

use crate::{
    functional::bbs::bbs_reach,
    node::{LowerBound, TreeNode},
};

/// Branch-and-Bound traversal implementation.
pub struct BranchAndBoundTraversal<N> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<N, C> BranchAndBoundTraversal<N>
where
    C: Copy + Ord + 'static,
    N: TreeNode<Cost = C> + LowerBound<Cost = C> + 'static,
{
    /// Creates a new BranchAndBoundTraversal starting from the given root node.
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
