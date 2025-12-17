//! Greedy Search

use std::time::Duration;

use super::{
    common::{NodeContainer, Reachable},
    find_best,
};

/// A container for Greedy traversal.
pub struct GreedyContainer<N, FN, FC> {
    next_node: Option<N>,
    successor_fn: FN,
    eval_fn: FC,
}

impl<N, IN, FN, FC, C> GreedyContainer<N, FN, FC>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> Option<C>,
    C: Ord + Copy,
{
    /// Creates a new `GreedyContainer` with the given parameters.
    pub fn new(start: N, successor_fn: FN, eval_fn: FC) -> Self {
        Self {
            next_node: Some(start),
            successor_fn,
            eval_fn,
        }
    }
}

impl<N, IN, FN, FC, C> NodeContainer for GreedyContainer<N, FN, FC>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> Option<C>,
    C: Ord + Copy,
{
    type Node = N;

    fn pop(&mut self) -> Option<Self::Node> {
        self.next_node.take()
    }

    fn expand_and_push(&mut self, node: &Self::Node) {
        let mut best_successor: Option<(C, N)> = None;
        for s in (self.successor_fn)(node) {
            if let Some(score) = (self.eval_fn)(&s) {
                match &best_successor {
                    Some((best_score, _)) => {
                        if score < *best_score {
                            best_successor = Some((score, s));
                        }
                    }
                    None => {
                        best_successor = Some((score, s));
                    }
                }
            }
        }

        self.next_node = best_successor.map(|(_, n)| n);
    }
}

/// Creates Greedy Search traversal iterator starting from the given node.
pub fn gds_reach<N, IN, FN, FC, C>(
    start: N,
    successor_fn: FN,
    eval_fn: FC,
) -> Reachable<GreedyContainer<N, FN, FC>>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FC: Fn(&N) -> Option<C>,
    C: Ord + Copy,
{
    let container = GreedyContainer::new(start, successor_fn, eval_fn);
    Reachable::new(container)
}

/// Find the leaf node with the lowest cost by using Greedy Search
pub fn gds<N, IN, FN, FC1, FC2, C, FR>(
    start: N,
    successor_fn: FN,
    leaf_check_fn: FR,
    cost_fn: FC2,
    eval_fn: FC1,
    max_ops: usize,
    time_limit: Duration,
) -> Option<(C, N)>
where
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FC1: Fn(&N) -> Option<C>,
    FC2: Fn(&N) -> Option<C>,
    C: Ord + Copy,
    FR: Fn(&N) -> bool,
{
    let mut res = gds_reach(start, successor_fn, eval_fn);
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

    use super::gds;

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

    #[test]
    fn test_gds() {
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
        let eval_fn = |n: &Node| Some(n.t);

        let cost_fn = |n: &Node| Some(n.t + time_func(n.city, start));
        let leaf_check_fn = |n: &Node| n.is_leaf();

        let max_ops = usize::MAX;
        let time_limit = std::time::Duration::from_secs(10);

        let (cost, best_node) = gds(
            root_node,
            successor_fn,
            leaf_check_fn,
            cost_fn,
            eval_fn,
            max_ops,
            time_limit,
        )
        .unwrap();

        assert!(cost < 9000);
        let mut visited_cities = best_node.parents.clone();
        visited_cities.push(best_node.city);
        visited_cities.sort();
        let all_cities: Vec<CityId> = (0..n_cities).collect();
        assert_eq!(visited_cities, all_cities);
    }
}
