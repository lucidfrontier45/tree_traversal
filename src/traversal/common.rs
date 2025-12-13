use crate::node::TreeNode;

/// Trait defining the interface for tree traversal algorithms.
pub trait Traversal {
    /// The type of nodes being traversed.
    type N: TreeNode;

    /// Returns the current best cost found during traversal, if any.
    fn current_best_cost(&self) -> Option<<Self::N as TreeNode>::Cost>;

    /// Sets the current best cost found during traversal.
    fn set_current_best_cost(&mut self, cost: <Self::N as TreeNode>::Cost);

    /// Performs a single step of the traversal algorithm, returning the next node to be processed, if any.
    fn step(&mut self) -> Option<Self::N>;

    /// Traverses the tree up to a maximum number of operations, returning all processed nodes.
    fn traverse(&mut self, max_ops: usize) -> Option<(<Self::N as TreeNode>::Cost, Self::N)> {
        let mut best_node = None;
        for _ in 0..max_ops {
            let Some(n) = self.step() else {
                break;
            };

            if !n.is_leaf() {
                continue;
            }

            let Some(cost) = n.cost() else {
                continue;
            };

            if let Some(current_best) = self.current_best_cost() {
                if cost < current_best {
                    best_node = Some(n);
                    self.set_current_best_cost(cost);
                }
            } else {
                best_node = Some(n);
                self.set_current_best_cost(cost);
            }
        }
        best_node.map(|n| (self.current_best_cost().unwrap(), n))
    }
}
