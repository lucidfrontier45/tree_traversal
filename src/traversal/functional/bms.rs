//! Beam Search

use std::{
    collections::{BinaryHeap, VecDeque},
    time::Duration,
};

use crate::utils::ScoredItem;

use super::common::{NodeContainer, Reachable, find_best};

/// A container for Beam Search traversal.
pub struct BeamContainer<N, FN, FP, P: Ord> {
    to_see: VecDeque<N>,
    successor_fn: FN,
    priority_fn: FP,
    branch_factor: usize,
    beam_width: usize,
    pool: BinaryHeap<ScoredItem<P, N>>,
}

impl<N, IN, FN, FP, P> BeamContainer<N, FN, FP, P>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FP: Fn(&N) -> Option<P>,
    P: Ord + Copy,
{
    /// Creates a new `BeamContainer` with the given parameters.
    pub fn new(
        start: N,
        successor_fn: FN,
        priority_fn: FP,
        branch_factor: usize,
        beam_width: usize,
    ) -> Self {
        Self {
            to_see: vec![start].into(),
            successor_fn,
            priority_fn,
            branch_factor,
            beam_width,
            pool: BinaryHeap::new(),
        }
    }
}

impl<N, IN, FN, FP, P> NodeContainer for BeamContainer<N, FN, FP, P>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FP: Fn(&N) -> Option<P>,
    P: Ord + Copy,
{
    type Node = N;

    fn pop(&mut self) -> Option<Self::Node> {
        if self.to_see.is_empty() {
            let max_iter = std::cmp::min(self.pool.len(), self.beam_width);
            for _ in 0..max_iter {
                if let Some(node) = self.pool.pop() {
                    self.to_see.push_back(node.into_item());
                }
            }
            self.pool.clear();
        }
        self.to_see.pop_front()
    }

    fn expand_and_push(&mut self, node: &Self::Node) {
        let mut successors: Vec<_> = (self.successor_fn)(node)
            .into_iter()
            .filter_map(|n| {
                let priority = (self.priority_fn)(&n)?;
                Some((priority, n))
            })
            .collect();
        successors.sort_by(|a, b| b.0.cmp(&a.0));
        successors
            .into_iter()
            .take(self.branch_factor)
            .for_each(|(score, n)| self.pool.push(ScoredItem::from((score, n))));
    }
}

/// Creates a Beam Search traversal iterator starting from the given node.
///
/// This function initializes a lazy iterator that explores the tree using beam search, maintaining
/// a fixed number of the most promising nodes (beam width) at each depth level. Nodes are selected
/// based on an evaluation function, and only the top successors are kept.
///
/// # Parameters
/// - `start`: The root node from which to begin the traversal.
/// - `successor_fn`: A function that, given a node, returns an iterator over its successor nodes.
/// - `priority_fn`: A function that evaluates a node for selection, returning `Some(priority)` where higher
///   priorities are better, or `None` if the node cannot be evaluated.
/// - `branch_factor`: The maximum number of successors to consider from each node.
/// - `beam_width`: The maximum number of nodes to keep at each depth level.
///
/// # Returns
/// An iterator that yields nodes reachable from the start node in beam search order.
/// The iterator is lazy and will only compute successors as needed.
pub fn bms_reach<N, IN, FN, FP, P>(
    start: N,
    successor_fn: FN,
    priority_fn: FP,
    branch_factor: usize,
    beam_width: usize,
) -> Reachable<BeamContainer<N, FN, FP, P>>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FP: Fn(&N) -> Option<P>,
    P: Ord + Copy,
{
    let container = BeamContainer::new(start, successor_fn, priority_fn, branch_factor, beam_width);
    Reachable::new(container)
}

/// Find the leaf node with the lowest cost by using Beam Search
///
/// - `start` is the start node.
/// - `successor_fn` returns a list of successors for a given node.
/// - `leaf_check_fn` checks if a node is a leaf or not
/// - `cost_fn` returns the final cost of a leaf node
/// - `priority_fn` returns the priority of a given node to sort and select k-best
/// - `branch_factor` decides maximum number of branches from a node
/// - `beam_width` decides maximum number of nodes at each depth.
/// - `max_ops` sets the maximum number of search operations to perform.
/// - `time_limit` sets the time limit for the search.
///
/// This function returns Some of a tuple of (cost, leaf node) if found, otherwise returns None
#[allow(clippy::too_many_arguments)]
pub fn bms<N, IN, FN, FC, FP, C, P, FR>(
    start: N,
    successor_fn: FN,
    leaf_check_fn: FR,
    cost_fn: FC,
    priority_fn: FP,
    branch_factor: usize,
    beam_width: usize,
    max_ops: usize,
    time_limit: Duration,
) -> Option<(C, N)>
where
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FC: Fn(&N) -> Option<C>,
    FP: Fn(&N) -> Option<P>,
    C: Ord + Copy,
    P: Ord + Copy,
    FR: Fn(&N) -> bool,
{
    let mut res = bms_reach(start, successor_fn, priority_fn, branch_factor, beam_width);
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
    use std::cmp::Reverse;

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
                    .expect("City should exist in children")
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
                .min_by_key(|x| x.1.0)
                .expect("Children should not be empty");

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
        let priority_fn = |n: &Node| {
            let (remained_duration, route) =
                greedy_tsp_solver(n.city, n.children.clone(), &time_func);
            Some(Reverse(
                n.t + remained_duration
                    + time_func(*route.last().expect("Route should not be empty"), start),
            ))
        };

        let branch_factor = 10;
        let beam_width = 5;
        let cost_fn = |n: &Node| Some(n.t + time_func(n.city, start));
        let leaf_check_fn = |n: &Node| n.is_leaf();

        let max_ops = usize::MAX;
        let time_limit = std::time::Duration::from_secs(10);

        let (cost, best_node) = bms(
            root_node,
            successor_fn,
            leaf_check_fn,
            cost_fn,
            priority_fn,
            branch_factor,
            beam_width,
            max_ops,
            time_limit,
        )
        .expect("BMS should find a valid solution");

        assert!(cost < 8000);
        let mut visited_cities = best_node.parents.clone();
        visited_cities.push(best_node.city);
        visited_cities.sort();
        let all_cities: Vec<CityId> = (0..n_cities).collect();
        assert_eq!(visited_cities, all_cities);
    }
}
