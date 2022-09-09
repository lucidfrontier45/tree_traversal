//! Branch and Bound Search

use std::iter::FusedIterator;

use num_traits::Bounded;
/// Struct returned by [`bbs_reach`]
pub struct BbsReachable<N, FN, FC, C> {
    to_see: Vec<N>,
    successor_fn: FN,
    lower_bound_fn: FC,
    current_best_cost: C,
}

impl<N, FN, IN, FC, C> Iterator for BbsReachable<N, FN, FC, C>
where
    N: Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> C,
    C: Ord + Copy + Bounded,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.to_see.pop() {
            if (self.lower_bound_fn)(&n) <= self.current_best_cost {
                for s in (self.successor_fn)(&n) {
                    self.to_see.push(s.clone());
                }
            }
            Some(n)
        } else {
            None
        }
    }
}

impl<N, FN, IN, FC, C> FusedIterator for BbsReachable<N, FN, FC, C>
where
    N: Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> C,
    C: Ord + Copy + Bounded,
{
}

/// Use Branch and Bound technique to efficiently traverse a tree
pub fn bbs_reach<N, FN, IN, FC, C>(
    start: N,
    successor_fn: FN,
    lower_bound_fn: FC,
) -> BbsReachable<N, FN, FC, C>
where
    N: Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> C,
    C: Ord + Copy + Bounded,
{
    BbsReachable {
        to_see: vec![start],
        successor_fn,
        lower_bound_fn,
        current_best_cost: C::max_value(),
    }
}

/// Find the leaf node with the lowest cost by using Branch and Bound
///
/// - `start` is the start node.
/// - `successor_fn` returns a list of successors for a given node.
/// - `lower_bound_fn` returns the lower bound of a given node do decide wheather search deeper or not
/// - `cost_fn` returns the final cost of a leaf node
/// - `leaf_check_fn` check if a node is leaf or not
///
/// This function returns Some of a tuple of (cost, leaf node) if found, otherwise returns None
pub fn bbs<N, IN, FN, FC1, FC2, C, FR>(
    start: N,
    successor_fn: FN,
    lower_bound_fn: FC1,
    cost_fn: FC2,
    leaf_check_fn: FR,
) -> Option<(C, N)>
where
    N: Clone,
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FC1: Fn(&N) -> C,
    FC2: Fn(&N) -> C,
    C: Ord + Copy + Bounded,
    FR: Fn(&N) -> bool,
{
    let mut res = bbs_reach(start, successor_fn, lower_bound_fn);
    let mut best_leaf_node = None;
    loop {
        let op_n = res.next();
        if op_n.is_none() {
            break;
        }
        let n = op_n.unwrap();
        if leaf_check_fn(&n) {
            let cost = cost_fn(&n);
            if res.current_best_cost > cost {
                res.current_best_cost = cost;
                best_leaf_node = Some(n)
            }
        }
    }

    best_leaf_node.and_then(|n| Some((res.current_best_cost, n)))
}

#[cfg(test)]
mod test {
    use super::bbs;
    type Node = Vec<bool>;
    #[test]
    fn test_bbs() {
        let weights = [4, 2, 6, 3, 4];
        let profits = [100, 20, 2, 5, 10];
        let capacity = 8 as u32;
        let total_items = weights.len();

        let successor_fn = |n: &Node| {
            if n.len() == total_items {
                return vec![];
            }

            let total_weight: u32 = n
                .iter()
                .copied()
                .enumerate()
                .map(|(i, b)| {
                    if b {
                        return weights[i];
                    } else {
                        return 0;
                    }
                })
                .sum();

            let mut childrean = vec![];

            let next_idx = n.len();
            if capacity >= total_weight + weights[next_idx] {
                let mut c1 = n.clone();
                c1.push(true);
                childrean.push(c1);
            }

            let mut c2 = n.clone();
            c2.push(false);
            childrean.push(c2);

            childrean
        };

        let total_profit = |n: &Node| {
            let s: u32 = n
                .iter()
                .copied()
                .enumerate()
                .map(|(i, b)| {
                    if b {
                        return profits[i];
                    } else {
                        return 0;
                    }
                })
                .sum();
            s
        };

        let lower_bound_fn = |n: &Node| {
            let current_profit = total_profit(n);
            let max_remained_profit: u32 = profits[n.len()..].into_iter().sum();
            u32::MAX - (current_profit + max_remained_profit)
        };

        let cost_fn = |n: &Node| u32::MAX - total_profit(n);

        let leaf_check_fn = |n: &Node| n.len() == total_items;

        let (cost, best_node) =
            bbs(vec![], successor_fn, lower_bound_fn, cost_fn, leaf_check_fn).unwrap();
        let cost = u32::MAX - cost;

        assert_eq!(cost, 120);
        assert_eq!(best_node, vec![true, true, false, false, false]);
    }
}
