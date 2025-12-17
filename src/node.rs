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
}

/// Trait defining the lower bound functionality for tree nodes.
pub trait LowerBound {
    /// The type representing the cost associated with the node.
    type Cost: Copy + Ord;

    /// Returns the lower bound associated with the node, if any.
    /// If the current node does not satisfy problem constraints, returns None.
    fn cost_lb(&self) -> Option<Self::Cost>;
}

/// Trait defining the priority functionality for tree nodes.
/// This is useful for greedy or best-first search algorithms.
pub trait Priority {
    /// The type representing the priority value associated with the node.
    /// Higher priority values indicate more favorable nodes.
    type Value: Copy + Ord;

    /// Returns the priority associated with the node, if any.
    /// If the current node does not satisfy problem constraints, returns None.
    fn priority(&self) -> Option<Self::Value>;
}
