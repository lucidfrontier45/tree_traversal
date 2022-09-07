//! Breadth First Search

use num_traits::Bounded;

use crate::bms::bms;

/// Find the best leaf node by using Breadth first search
pub fn bfs<N, IN, FN, FC, C, FR>(
    start: N,
    successor_fn: FN,
    score_fn: FC,
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
        score_fn,
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

        let score_fn = |n: &Node| {
            let score: u32 = n
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
            u32::MAX - score
        };

        let leaf_check_fn = |n: &Node| n.len() == total_items;

        let (score, best_node) = bfs(vec![], successor_fn, score_fn, leaf_check_fn);
        let score = u32::MAX - score;

        assert_eq!(score, 6);
        assert_eq!(best_node, vec![true, false, false, true]);
    }
}
