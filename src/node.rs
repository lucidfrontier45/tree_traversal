//! Defines the TreeNode trait for tree traversal nodes.

/// TreeNode trait that implements basic functionalities for tree traversal nodes.
pub trait TreeNode: Sized {
    /// The type representing the cost associated with the node.
    type Cost: Copy + Ord;

    /// Checks if the node is a leaf node.
    fn is_leaf(&self) -> bool;
    /// Generates child nodes from the current node.
    fn generate_child_nodes(&self) -> Vec<Self>;
    /// Returns the cost associated with the node, if any.
    /// If the current node does not satisfy problem constraints, returns None.
    fn cost(&self) -> Option<Self::Cost>;
    /// Returns the lower bound associated with the node, if any.
    /// If the current node does not satisfy problem constraints, returns None.
    fn cost_lb(&self) -> Option<Self::Cost> {
        self.cost()
    }
    /// Returns the approximate cost associated with the node, if any.
    /// If the current node does not satisfy problem constraints, returns None.
    fn cost_approx(&self) -> Option<Self::Cost> {
        self.cost()
    }
}
