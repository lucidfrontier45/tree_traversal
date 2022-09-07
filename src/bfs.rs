//! Breadth First Search

use num_traits::Bounded;

use crate::bms::bms;

/// Find the leaf node with the lowest cost by using Breadth First Search
///
/// - `start` is the start node.
/// - `successor_fn` returns a list of successors for a given node.
/// - `cost_fn` returns the final cost of a leaf node
/// - `leaf_check_fn` check if a node is leaf or not
///
/// This function returns Some of a tuple of (cost, leaf node) if found, otherwise returns None
pub fn bfs<N, IN, FN, FC, C, FR>(
    start: N,
    successor_fn: FN,
    cost_fn: FC,
    leaf_check_fn: FR,
) -> (C, N)
where
    N: Clone,
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FC: Fn(&N) -> C,
    C: Ord + Copy + Bounded,
    FR: Fn(&N) -> bool,
{
    bms(
        start,
        successor_fn,
        |_| C::min_value(),
        usize::MAX,
        usize::MAX,
        cost_fn,
        leaf_check_fn,
    )
}

#[cfg(test)]
mod test {
    use super::bfs;
    type Node = Vec<bool>;
    #[test]
    fn test_bfs() {
        let weights = [3, 4, 6, 5];
        let profits = [2, 3, 2, 4];
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
                .map(|(i, b)| {
                    if b {
                        return profits[i];
                    } else {
                        return 0;
                    }
                })
                .sum();
            u32::MAX - cost
        };

        let leaf_check_fn = |n: &Node| n.len() == total_items;

        let (cost, best_node) = bfs(vec![], successor_fn, cost_fn, leaf_check_fn);
        let cost = u32::MAX - cost;

        assert_eq!(cost, 6);
        assert_eq!(best_node, vec![true, false, false, true]);
    }
}
