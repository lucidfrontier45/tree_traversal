//! Tree traversal algorithms and iterator-based traversal adapters.
//!
//! This module provides several traversal implementations (BFS, DFS, Beam, Greedy,
//! Branch-and-Bound, Priority-First) as both iterator-based adapters and functional
//! helpers under the `functional` submodule. The implementations are generic over a
//! `TreeNode` trait so they can be reused for different problem domains (e.g., knapsack,
//! TSP).

mod bbs;
mod bfs;
mod bms;
mod common;
mod dfs;
mod gds;
mod pfs;

pub mod functional;

pub use bbs::BranchAndBoundTraversal;
pub use bfs::BreadthFirstTraversal;
pub use bms::BeamTraversal;
pub use common::{Traversal, find_best, traverse};
pub use dfs::DepthFirstTraversal;
pub use gds::GreedyTraversal;
pub use pfs::PriorityFirstTraversal;
