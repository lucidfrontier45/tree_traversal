use std::iter::FusedIterator;

use crate::{
    functional::bms_reach,
    node::{Approximate, TreeNode},
};

/// Beam traversal implementation.
pub struct BeamTraversal<N> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<C, N> BeamTraversal<N>
where
    C: Copy + Ord + 'static,
    N: TreeNode<Cost = C> + Approximate<Cost = C> + 'static,
{
    /// Creates a new BeamTraversal starting from the given root node.
    pub fn new(root_node: N, branch_factor: usize, beam_width: usize) -> Self {
        let state = bms_reach(
            root_node,
            |n: &N| n.generate_child_nodes(),
            |n: &N| n.cost_approx(),
            branch_factor,
            beam_width,
        );
        Self {
            state: Box::new(state),
        }
    }
}

impl<N> Iterator for BeamTraversal<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next()
    }
}
