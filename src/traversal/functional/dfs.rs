//! Depth First Search

use std::time::Duration;

use super::{
    common::{NodeContainer, Reachable},
    find_best,
};

/// A container for Depth-First traversal.
pub struct DepthFirstContainer<N, FN> {
    to_see: Vec<N>,
    successor_fn: FN,
}

impl<N, FN, IN> DepthFirstContainer<N, FN>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    /// Creates a new `DepthFirstContainer` with the given successor function.
    pub fn new(start: N, successor_fn: FN) -> Self {
        Self {
            to_see: vec![start],
            successor_fn,
        }
    }
}

impl<N, FN, IN> NodeContainer for DepthFirstContainer<N, FN>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    type Node = N;

    fn pop(&mut self) -> Option<Self::Node> {
        self.to_see.pop()
    }

    fn expand_and_push(&mut self, node: &Self::Node) {
        for s in (self.successor_fn)(node) {
            self.to_see.push(s);
        }
    }
}

/// Creates a Depth-First Search traversal iterator starting from the given node.
///
/// This function initializes a lazy iterator that explores the tree by going as deep as possible
/// along each branch before backtracking, yielding nodes in depth-first order.
///
/// # Parameters
/// - `start`: The root node from which to begin the traversal.
/// - `successor_fn`: A function that, given a node, returns an iterator over its successor nodes.
///
/// # Returns
/// An iterator that yields nodes reachable from the start node in depth-first order.
/// The iterator is lazy and will only compute successors as needed.
pub fn dfs_reach<N, IN, FN>(start: N, successor_fn: FN) -> Reachable<DepthFirstContainer<N, FN>>
where
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
{
    let container = DepthFirstContainer::new(start, successor_fn);
    Reachable::new(container)
}

/// Find the leaf node with the lowest cost by using Depth First Search
///
/// - `start` is the start node.
/// - `successor_fn` returns a list of successors for a given node.
/// - `leaf_check_fn` check if a node is leaf or not
/// - `cost_fn` returns the final cost of a leaf node
/// - `max_ops` is the maximum number of search operations to perform
/// - `time_limit` is the maximum duration allowed for the search operation
///
/// This function returns Some of a tuple of (cost, leaf node) if found, otherwise returns None
pub fn dfs<N, IN, FN, FC, C, FL>(
    start: N,
    successor_fn: FN,
    leaf_check_fn: FL,
    cost_fn: FC,
    max_ops: usize,
    time_limit: Duration,
) -> Option<(C, N)>
where
    C: Ord + Copy + Default,
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FL: Fn(&N) -> bool,
    FC: Fn(&N) -> Option<C>,
{
    let mut res = dfs_reach(start, successor_fn);
    find_best(
        &mut res,
        leaf_check_fn,
        cost_fn,
        max_ops,
        time_limit,
        |_, _| {},
    )
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::dfs;
    type Node = Vec<bool>;
    #[test]
    fn test_dfs() {
        let weights = [3, 4, 6, 5];
        let profits = [2, 3, 2, 4];
        let capacity = 8;
        let total_items = weights.len();

        let successor_fn = |n: &Node| {
            if n.len() == total_items {
                return vec![];
            }

            let total_weight: u32 = n
                .iter()
                .copied()
                .enumerate()
                .map(|(i, b)| if b { weights[i] } else { 0 })
                .sum();

            let mut children = vec![];

            let mut c1 = n.clone();
            c1.push(false);
            children.push(c1);

            let next_idx = n.len();
            if capacity >= total_weight + weights[next_idx] {
                let mut c2 = n.clone();
                c2.push(true);
                children.push(c2);
            }

            children
        };

        let cost_fn = |n: &Node| {
            let cost: u32 = n
                .iter()
                .copied()
                .enumerate()
                .map(|(i, b)| if b { profits[i] } else { 0 })
                .sum();
            Some(u32::MAX - cost)
        };

        let leaf_check_fn = |n: &Node| n.len() == total_items;
        let max_ops = usize::MAX;
        let time_limit = Duration::from_secs(10);

        let (cost, best_node) = dfs(
            vec![],
            successor_fn,
            leaf_check_fn,
            cost_fn,
            max_ops,
            time_limit,
        )
        .expect("DFS should find a valid solution");
        let cost = u32::MAX - cost;

        assert_eq!(cost, 6);
        assert_eq!(best_node, vec![true, false, false, true]);
    }
}
