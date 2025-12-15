use std::iter::FusedIterator;

use crate::{functional::bbs::bbs_reach, node::TreeNode};

/// Branch-and-Bound traversal implementation.
pub struct DepthFirstTraversal<N: TreeNode> {
    state: Box<dyn FusedIterator<Item = N>>,
}

impl<C: Copy + Ord + Default + 'static, N: TreeNode<Cost = C> + 'static> DepthFirstTraversal<N> {
    /// Creates a new BranchAndBoundTraversal starting from the given root node.
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

impl<C: Copy + Ord + 'static, N: TreeNode<Cost = C> + 'static> Iterator for DepthFirstTraversal<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next()
    }
}
