use std::hash::Hash;

use pathfinding::prelude::dfs_reach;

pub fn dfs<N, IN, FN, FC, C, FR>(
    start: N,
    successor_fn: FN,
    score_fn: FC,
    root_check_fn: FR,
) -> (C, N)
where
    N: Eq + Hash + Clone,
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FC: Fn(&N) -> C,
    C: Ord + Copy,
    FR: Fn(&N) -> bool,
{
    let res = dfs_reach(start, successor_fn);
    let (score, best_node) = res
        .into_iter()
        .filter_map(|n| {
            if !root_check_fn(&n) {
                return None;
            } else {
                return Some((score_fn(&n), n));
            }
        })
        .min_by_key(|x| x.0)
        .unwrap();

    (score, best_node)
}

#[cfg(test)]
mod test {
    use super::dfs;
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

        let root_check_fn = |n: &Node| n.len() == total_items;

        let (score, best_node) = dfs(vec![], successor_fn, score_fn, root_check_fn);
        let score = u32::MAX - score;

        assert_eq!(score, 6);
        assert_eq!(best_node, vec![true, false, false, true]);
    }
}