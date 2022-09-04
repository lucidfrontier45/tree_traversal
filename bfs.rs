use std::hash::Hash;

use pathfinding::prelude::bfs_reach;

pub fn bfs<N, IN, FN, FC, C>(start: N, successor_fn: FN, score_fn: FC) -> (C, N)
where
    N: Eq + Hash + Clone,
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FC: Fn(&N) -> C,
    C: Ord + Copy,
{
    let res = bfs_reach(start, successor_fn);
    let (score, best_node) = res
        .into_iter()
        .map(|n| (score_fn(&n), n))
        .min_by_key(|x| x.0)
        .unwrap();

    (score, best_node)
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

        let successor_fn = |n: &Node| {
            if n.len() == 4 {
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
            if n.len() < 4 {
                return u32::MAX;
            }
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

        let (score, best_node) = bfs(vec![], successor_fn, score_fn);
        let score = u32::MAX - score;

        assert_eq!(score, 6);
        assert_eq!(best_node, vec![true, false, false, true]);
    }
}
