use crate::{
    node::{LowerBound, TreeNode},
    traversal::common::Traversal,
};

/// Branch-and-Bound traversal implementation.
pub struct BranchAndBoundTraversal<N: TreeNode + LowerBound> {
    to_see: Vec<N>,
    current_best_cost: Option<<N as TreeNode>::Cost>,
}

impl<C: Copy + Ord, N: TreeNode<Cost = C> + LowerBound<Cost = C>> BranchAndBoundTraversal<N> {
    /// Creates a new BranchAndBoundTraversal starting from the given root node.
    pub fn new(root_node: N) -> Self {
        let to_see = vec![root_node];
        Self {
            to_see,
            current_best_cost: None,
        }
    }
}

impl<C: Copy + Ord, N: TreeNode<Cost = C> + LowerBound<Cost = C>> Traversal
    for BranchAndBoundTraversal<N>
{
    type Node = N;

    fn current_best_cost(&self) -> Option<<N as TreeNode>::Cost> {
        self.current_best_cost
    }

    fn set_current_best_cost(&mut self, cost: <N as TreeNode>::Cost) {
        self.current_best_cost = Some(cost);
    }

    fn step(&mut self) -> Option<Self::Node> {
        let node = self.to_see.pop()?;

        if let Some(lb) = node.cost_lb() {
            if self.current_best_cost().is_none_or(|c| c >= lb) {
                for child in node.generate_child_nodes() {
                    self.to_see.push(child);
                }
            }
        }
        Some(node)
    }
}
