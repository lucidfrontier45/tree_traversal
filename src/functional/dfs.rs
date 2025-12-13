//! Depth First Search

use std::time::Duration;

use crate::{bbs::bbs_reach, common::search};

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
    N: Clone,
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FL: Fn(&N) -> bool,
    FC: Fn(&N) -> Option<C>,
{
    let mut res = bbs_reach(
        start,
        successor_fn,
        |_| false,
        |_| None,
        |_| Some(C::default()),
    );
    search(&mut res, leaf_check_fn, cost_fn, max_ops, time_limit)
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

            let mut childrean = vec![];

            let mut c1 = n.clone();
            c1.push(false);
            childrean.push(c1);

            let next_idx = n.len();
            if capacity >= total_weight + weights[next_idx] {
                let mut c2 = n.clone();
                c2.push(true);
                childrean.push(c2);
            }

            childrean
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
        .unwrap();
        let cost = u32::MAX - cost;

        assert_eq!(cost, 6);
        assert_eq!(best_node, vec![true, false, false, true]);
    }
}
