use std::{
    collections::{BinaryHeap, VecDeque},
    iter::FusedIterator,
};

use num_traits::Bounded;

struct ScoredItem<S: Ord, A>(S, A);

impl<S: Ord, A> PartialEq for ScoredItem<S, A> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S: Ord, A> Eq for ScoredItem<S, A> {}

impl<S: Ord, A> PartialOrd for ScoredItem<S, A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0).map(|ord| ord.reverse())
    }
}

impl<S: Ord, A> Ord for ScoredItem<S, A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl<S: Ord, A> From<(S, A)> for ScoredItem<S, A> {
    fn from(v: (S, A)) -> Self {
        let (score, item) = v;
        ScoredItem(score, item)
    }
}

/// Struct returned by [`dfs_reach`](crate::directed::dfs::dfs_reach).
pub struct BmsReachable<N, FN, FC, C: Ord> {
    to_see: VecDeque<N>,
    successor_fn: FN,
    eval_fn: FC,
    branch_factor: usize,
    beam_width: usize,
    pool: BinaryHeap<ScoredItem<C, N>>,
}

impl<N, FN, IN, FC, C> Iterator for BmsReachable<N, FN, FC, C>
where
    N: Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> C,
    C: Ord + Copy + Bounded,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_see.is_empty() {
            let max_iter = std::cmp::min(self.pool.len(), self.beam_width);
            for _ in 0..max_iter {
                self.to_see.push_back(self.pool.pop().unwrap().1);
            }
            self.pool.clear();
        }
        if let Some(node) = self.to_see.pop_front() {
            let mut successors: Vec<_> = (self.successor_fn)(&node)
                .into_iter()
                .map(|n| ((self.eval_fn)(&n), n))
                .collect();
            successors.sort_unstable_by_key(|x| x.0);
            successors
                .into_iter()
                .take(self.branch_factor)
                .for_each(|item| self.pool.push(item.into()));
            Some(node)
        } else {
            None
        }
    }
}

impl<N, FN, IN, FC, C> FusedIterator for BmsReachable<N, FN, FC, C>
where
    N: Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> C,
    C: Ord + Copy + Bounded,
{
}

pub fn bms_reach<N, FN, IN, FC, C>(
    start: N,
    successor_fn: FN,
    eval_fn: FC,
    branch_factor: usize,
    beam_width: usize,
) -> BmsReachable<N, FN, FC, C>
where
    N: Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> C,
    C: Ord + Copy + Bounded,
{
    BmsReachable {
        to_see: vec![start].into(),
        successor_fn,
        eval_fn,
        branch_factor,
        beam_width,
        pool: BinaryHeap::new(),
    }
}

pub fn bms<N, IN, FN, FC1, FC2, C, FR>(
    start: N,
    successor_fn: FN,
    eval_fn: FC1,
    branch_factor: usize,
    beam_width: usize,
    score_fn: FC2,
    leaf_check_fn: FR,
) -> (C, N)
where
    N: Clone,
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FC1: Fn(&N) -> C,
    FC2: Fn(&N) -> C,
    C: Ord + Copy + Bounded,
    FR: Fn(&N) -> bool,
{
    let mut res = bms_reach(start, successor_fn, eval_fn, branch_factor, beam_width);
    let mut best_root_node = None;
    let mut current_best_score = C::max_value();
    loop {
        let op_n = res.next();
        if op_n.is_none() {
            break;
        }
        let n = op_n.unwrap();
        if leaf_check_fn(&n) {
            let score = score_fn(&n);
            if current_best_score > score {
                current_best_score = score;
                best_root_node = Some(n)
            }
        }
    }

    (current_best_score, best_root_node.unwrap())
}

#[cfg(test)]
mod test {

    use super::bms;

    type CityId = usize;
    type Duration = u32;

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Node {
        pub city: CityId,
        pub parents: Vec<CityId>,
        pub children: Vec<CityId>,
        pub t: Duration,
    }

    impl Node {
        pub fn new(city: CityId, parents: Vec<CityId>, children: Vec<CityId>, t: Duration) -> Self {
            Self {
                city,
                parents,
                children,
                t,
            }
        }

        pub fn from_parent(
            parent: &Self,
            city: CityId,
            time_func: &dyn Fn(CityId, CityId) -> Duration,
        ) -> Self {
            let parents = {
                let mut _parents = parent.parents.clone();
                _parents.push(parent.city);
                _parents
            };

            let children = {
                let mut _children = parent.children.clone();
                let i = _children
                    .iter()
                    .copied()
                    .enumerate()
                    .find(|&(_, c)| c == city)
                    .unwrap()
                    .0;
                _children.remove(i);
                _children
            };

            let t = parent.t + time_func(parent.city, city);

            Node {
                city,
                parents,
                children,
                t,
            }
        }

        pub fn is_leaf(&self) -> bool {
            self.children.is_empty()
        }

        pub fn generate_child_nodes(
            &self,
            time_func: &dyn Fn(CityId, CityId) -> Duration,
        ) -> Vec<Self> {
            let mut child_nodes = vec![];
            for city in self.children.iter().copied() {
                let node = Self::from_parent(self, city, time_func);
                child_nodes.push(node);
            }
            child_nodes
        }
    }

    fn greedy_tsp_solver(
        start: CityId,
        mut cities: Vec<CityId>,
        time_func: &dyn Fn(CityId, CityId) -> Duration,
    ) -> (u32, Vec<usize>) {
        let n = cities.len();
        let mut route = vec![start];
        let mut prev_city = start;
        let mut total_dist = 0;
        for _ in 0..n {
            let (i, (dist, next_city)) = cities
                .iter()
                .map(|c| (time_func(prev_city, *c), *c))
                .enumerate()
                .min_by_key(|x| x.1 .0)
                .unwrap();

            cities.remove(i);
            route.push(next_city);
            prev_city = next_city;
            total_dist += dist;
        }

        (total_dist, route)
    }

    #[test]
    fn test_bms() {
        let distance_matrix = [
            [
                0, 2451, 713, 1018, 1631, 1374, 2408, 213, 2571, 875, 1420, 2145, 1972,
            ],
            [
                2451, 0, 1745, 1524, 831, 1240, 959, 2596, 403, 1589, 1374, 357, 579,
            ],
            [
                713, 1745, 0, 355, 920, 803, 1737, 851, 1858, 262, 940, 1453, 1260,
            ],
            [
                1018, 1524, 355, 0, 700, 862, 1395, 1123, 1584, 466, 1056, 1280, 987,
            ],
            [
                1631, 831, 920, 700, 0, 663, 1021, 1769, 949, 796, 879, 586, 371,
            ],
            [
                1374, 1240, 803, 862, 663, 0, 1681, 1551, 1765, 547, 225, 887, 999,
            ],
            [
                2408, 959, 1737, 1395, 1021, 1681, 0, 2493, 678, 1724, 1891, 1114, 701,
            ],
            [
                213, 2596, 851, 1123, 1769, 1551, 2493, 0, 2699, 1038, 1605, 2300, 2099,
            ],
            [
                2571, 403, 1858, 1584, 949, 1765, 678, 2699, 0, 1744, 1645, 653, 600,
            ],
            [
                875, 1589, 262, 466, 796, 547, 1724, 1038, 1744, 0, 679, 1272, 1162,
            ],
            [
                1420, 1374, 940, 1056, 879, 225, 1891, 1605, 1645, 679, 0, 1017, 1200,
            ],
            [
                2145, 357, 1453, 1280, 586, 887, 1114, 2300, 653, 1272, 1017, 0, 504,
            ],
            [
                1972, 579, 1260, 987, 371, 999, 701, 2099, 600, 1162, 1200, 504, 0,
            ],
        ];

        let n_cities = distance_matrix.len();

        let start = 0;
        let root_node = Node::new(start, vec![], (1..n_cities).collect(), 0);
        let time_func = |p: CityId, c: CityId| distance_matrix[p][c];

        let successor_fn = |n: &Node| n.generate_child_nodes(&time_func);
        let eval_fn = |n: &Node| {
            let (remained_duration, route) =
                greedy_tsp_solver(n.city, n.children.clone(), &time_func);
            n.t + remained_duration + time_func(*route.last().unwrap(), start)
        };

        let branch_factor = 10;
        let beam_width = 5;
        let score_fn = |n: &Node| n.t + time_func(n.city, start);
        let leaf_check_fn = |n: &Node| n.is_leaf();

        let (score, best_node) = bms(
            root_node,
            successor_fn,
            eval_fn,
            branch_factor,
            beam_width,
            score_fn,
            leaf_check_fn,
        );

        assert!(score < 8000);
        let mut visited_cities = best_node.parents.clone();
        visited_cities.push(best_node.city);
        visited_cities.sort();
        let all_cities: Vec<CityId> = (0..n_cities).into_iter().collect();
        assert_eq!(visited_cities, all_cities);
    }
}