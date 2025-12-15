use std::iter::FusedIterator;

use crate::{
    functional::bbs::BbsReachable,
    node::{LowerBound, TreeNode},
};

/// Branch-and-Bound traversal implementation.
pub struct BranchAndBoundTraversal<N: TreeNode + LowerBound> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<C: Copy + Ord + 'static, N: TreeNode<Cost = C> + LowerBound<Cost = C> + 'static>
    BranchAndBoundTraversal<N>
{
    /// Creates a new BranchAndBoundTraversal starting from the given root node.
    pub fn new(root_node: N) -> Self {
        let state = BbsReachable::new(
            vec![root_node],
            |n: &N| n.generate_child_nodes(),
            |n: &N| n.is_leaf(),
            |n: &N| n.cost(),
            |n: &N| n.cost_lb(),
            None,
        );
        Self {
            state: Box::new(state),
        }
    }
}

impl<C: Copy + Ord + 'static, N: TreeNode<Cost = C> + LowerBound<Cost = C> + 'static> Iterator
    for BranchAndBoundTraversal<N>
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next()
    }
}
