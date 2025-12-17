//! Branch and Bound Search

use std::time::Duration;

use super::{
    common::{NodeContainer, Reachable},
    find_best,
};

/// A container for Branch-and-Bound traversal.
pub struct BranchAndBoundContainer<C, N, FN, FL, FC, FC2> {
    to_see: Vec<N>,
    successor_fn: FN,
    leaf_check_fn: FL,
    cost_fn: FC,
    lower_bound_fn: FC2,
    current_best_cost: Option<C>,
}

impl<C, N, IN, FN, FL, FC, FC2> BranchAndBoundContainer<C, N, FN, FL, FC, FC2>
where
    C: Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FL: Fn(&N) -> bool,
    FC: Fn(&N) -> Option<C>,
    FC2: Fn(&N) -> Option<C>,
{
    /// Creates a new `BranchAndBoundContainer` with the given parameters.
    pub fn new(
        start: N,
        successor_fn: FN,
        leaf_check_fn: FL,
        cost_fn: FC,
        lower_bound_fn: FC2,
    ) -> Self {
        Self {
            to_see: vec![start],
            successor_fn,
            leaf_check_fn,
            cost_fn,
            lower_bound_fn,
            current_best_cost: None,
        }
    }
}

impl<C, N, FN, FL, FC, FC2, IN> NodeContainer for BranchAndBoundContainer<C, N, FN, FL, FC, FC2>
where
    C: Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FL: Fn(&N) -> bool,
    FC: Fn(&N) -> Option<C>,
    FC2: Fn(&N) -> Option<C>,
{
    type Node = N;

    fn pop(&mut self) -> Option<Self::Node> {
        self.to_see.pop()
    }

    fn expand_and_push(&mut self, node: &Self::Node) {
        if (self.leaf_check_fn)(node) {
            if let Some(cost) = (self.cost_fn)(node)
                && self.current_best_cost.is_none_or(|c| c > cost)
            {
                self.current_best_cost = Some(cost);
            }
        } else if let Some(lb) = (self.lower_bound_fn)(node)
            && self.current_best_cost.is_none_or(|c| c > lb)
        {
            for s in (self.successor_fn)(node) {
                self.to_see.push(s);
            }
        }
    }
}

/// Creates a Branch-and-Bound traversal iterator starting from the given node.
///
/// This function initializes a lazy iterator that explores the tree using the Branch-and-Bound algorithm,
/// pruning branches based on lower bounds and the current best cost. Nodes are yielded in an order that
/// prioritizes promising paths with lower estimated costs.
///
/// # Parameters
/// - `start`: The root node from which to begin the traversal.
/// - `successor_fn`: A function that, given a node, returns an iterator over its successor nodes.
/// - `leaf_check_fn`: A function that determines whether a given node is a leaf (terminal) node.
/// - `cost_fn`: A function that computes the cost of a leaf node, returning `Some(cost)` if the cost
///   can be determined, or `None` otherwise.
/// - `lower_bound_fn`: A function that provides a lower bound on the cost for a given node, used
///   for pruning suboptimal branches.
///
/// # Returns
/// An iterator that yields nodes reachable from the start node in Branch-and-Bound order.
/// The iterator is lazy and will only compute successors as needed.
pub fn bbs_reach<C, N, IN, FN, FL, FC, FC2>(
    start: N,
    successor_fn: FN,
    leaf_check_fn: FL,
    cost_fn: FC,
    lower_bound_fn: FC2,
) -> Reachable<BranchAndBoundContainer<C, N, FN, FL, FC, FC2>>
where
    C: Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> Option<C>,
    FL: Fn(&N) -> bool,
    FC2: Fn(&N) -> Option<C>,
{
    let container =
        BranchAndBoundContainer::new(start, successor_fn, leaf_check_fn, cost_fn, lower_bound_fn);
    Reachable::new(container)
}

/// Find the leaf node with the lowest cost by using Branch and Bound
///
/// - `start` is the start node.
/// - `successor_fn` returns a list of successors for a given node.
/// - `leaf_check_fn` check if a node is leaf or not
/// - `cost_fn` returns the final cost of a leaf node
/// - `lower_bound_fn` returns the lower bound of a given node to decide whether to search deeper or not
/// - `max_ops` is the maximum number of search operations to perform
/// - `time_limit` is the maximum duration allowed for the search operation
///
/// This function returns Some of a tuple of (cost, leaf node) if found, otherwise returns None
pub fn bbs<C, N, IN, FN, FL, FC, FC2>(
    start: N,
    successor_fn: FN,
    leaf_check_fn: FL,
    cost_fn: FC,
    lower_bound_fn: FC2,
    max_ops: usize,
    time_limit: Duration,
) -> Option<(C, N)>
where
    C: Ord + Copy,
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FC: Copy + Fn(&N) -> Option<C>,
    FL: Copy + Fn(&N) -> bool,
    FC2: Fn(&N) -> Option<C>,
{
    let mut res = bbs_reach(start, successor_fn, leaf_check_fn, cost_fn, lower_bound_fn);
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

    use super::bbs;
    type Node = Vec<bool>;
    #[test]
    fn test_bbs() {
        let weights = [4, 2, 6, 3, 4];
        let profits = [100, 20, 2, 5, 10];
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

            let next_idx = n.len();
            if capacity >= total_weight + weights[next_idx] {
                let mut c1 = n.clone();
                c1.push(true);
                children.push(c1);
            }

            let mut c2 = n.clone();
            c2.push(false);
            children.push(c2);

            children
        };

        let total_profit = |n: &Node| {
            let s: u32 = n
                .iter()
                .copied()
                .enumerate()
                .map(|(i, b)| if b { profits[i] } else { 0 })
                .sum();
            s
        };

        let lower_bound_fn = |n: &Node| {
            let current_profit = total_profit(n);
            let max_remained_profit: u32 = profits[n.len()..].iter().sum();
            Some(u32::MAX - (current_profit + max_remained_profit))
        };

        let cost_fn = |n: &Node| Some(u32::MAX - total_profit(n));

        let leaf_check_fn = |n: &Node| n.len() == total_items;

        let max_ops = usize::MAX;
        let time_limit = Duration::from_secs(10);

        let (cost, best_node) = bbs(
            vec![],
            successor_fn,
            leaf_check_fn,
            cost_fn,
            lower_bound_fn,
            max_ops,
            time_limit,
        )
        .unwrap();
        let cost = u32::MAX - cost;

        assert_eq!(cost, 120);
        assert_eq!(best_node, vec![true, true, false, false, false]);
    }
}
