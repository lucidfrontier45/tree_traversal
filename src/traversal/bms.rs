use std::iter::FusedIterator;

use crate::node::{Approximate, TreeNode};

use super::functional::bms_reach;

/// Beam traversal implementation.
pub struct BeamTraversal<N> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<C, N> BeamTraversal<N>
where
    C: Copy + Ord + 'static,
    N: TreeNode<Cost = C> + Approximate<Cost = C> + 'static,
{
    /// Creates a new `BeamTraversal` instance that performs a beam search starting from the given root node.
    ///
    /// Beam search is a heuristic search algorithm that explores the most promising nodes in a breadth-first manner,
    /// maintaining a fixed-size "beam" of the best candidates at each level to limit memory usage.
    ///
    /// # Parameters
    /// - `root_node`: The starting node for the traversal.
    /// - `branch_factor`: The maximum number of child nodes to consider per parent node.
    /// - `beam_width`: The maximum number of nodes to keep in the beam at each level.
    ///
    /// # Returns
    /// A new `BeamTraversal` iterator.
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

impl<N> FusedIterator for BeamTraversal<N> {}
