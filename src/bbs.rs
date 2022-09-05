use std::{collections::HashSet, hash::Hash, iter::FusedIterator};

/// Struct returned by [`dfs_reach`](crate::directed::dfs::dfs_reach).
pub struct BbsReachable<N, FN, FC, C> {
    to_see: Vec<N>,
    seen: HashSet<N>,
    successors: FN,
    lower_bound_fn: FC,
    current_best_score: Option<C>,
}

impl<N, FN, IN, FC, C> Iterator for BbsReachable<N, FN, FC, C>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> C,
    C: Ord + Copy,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.to_see.pop() {
            if self.current_best_score.is_none()
                || (self.lower_bound_fn)(&n) <= self.current_best_score.unwrap()
            {
                let mut to_insert = Vec::new();
                for s in (self.successors)(&n) {
                    if !self.seen.contains(&s) {
                        to_insert.push(s.clone());
                        self.seen.insert(s);
                    }
                }
                self.to_see.extend(to_insert.into_iter().rev());
            }
            Some(n)
        } else {
            None
        }
    }
}

impl<N, FN, IN, FC, C> FusedIterator for BbsReachable<N, FN, FC, C>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> C,
    C: Ord + Copy,
{
}

pub fn bbs_reach<N, FN, IN, FC, C>(
    start: N,
    successors: FN,
    lower_bound_fn: FC,
    current_best_score: Option<C>,
) -> BbsReachable<N, FN, FC, C>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> C,
    C: Ord + Copy,
{
    let (to_see, mut seen) = (vec![start.clone()], HashSet::new());
    seen.insert(start);
    BbsReachable {
        to_see,
        seen,
        successors,
        lower_bound_fn,
        current_best_score,
    }
}

pub fn bbs<N, IN, FN, FC1, FC2, C, FR>(
    start: N,
    successor_fn: FN,
    lower_bound_fn: FC1,
    score_fn: FC2,
    root_check_fn: FR,
) -> (C, N)
where
    N: Eq + Hash + Clone,
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FC1: Fn(&N) -> C,
    FC2: Fn(&N) -> C,
    C: Ord + Copy,
    FR: Fn(&N) -> bool,
{
    let mut res = bbs_reach(start, successor_fn, lower_bound_fn, None);
    let mut best_root_node = None;
    loop {
        let op_n = res.next();
        if op_n.is_none() {
            break;
        }
        let n = op_n.unwrap();
        if root_check_fn(&n) {
            let score = score_fn(&n);
            if res.current_best_score.is_none() || res.current_best_score.unwrap() > score {
                res.current_best_score = Some(score);
                best_root_node = Some(n)
            }
        }
    }

    (res.current_best_score.unwrap(), best_root_node.unwrap())
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

        let score_fn = |n: &Node| u32::MAX - total_profit(n);

        let root_check_fn = |n: &Node| n.len() == total_items;

        let (score, best_node) = bbs(
            vec![],
            successor_fn,
            lower_bound_fn,
            score_fn,
            root_check_fn,
        );
        let score = u32::MAX - score;

        assert_eq!(score, 120);
        assert_eq!(best_node, vec![true, true, false, false, false]);
    }
}
