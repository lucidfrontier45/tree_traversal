//! Greedy Search

use num_traits::Bounded;

use crate::bms::bms;

/// Findthe leaf node with the lowest cost by using Greedy Search
///
/// - `start` is the start node.
/// - `successor_fn` returns a list of successors for a given node.
/// - `eval_fn` returns the approximated cost of a given node to sort and select k-best
/// - `cost_fn` returns the final cost of a leaf node
/// - `leaf_check_fn` check if a node is leaf or not
///
/// This function returns Some of a tuple of (cost, leaf node) if found, otherwise returns None
pub fn gds<N, IN, FN, FC1, FC2, C, FR>(
    start: N,
    successor_fn: FN,
    eval_fn: FC1,
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
    bms(
        start,
        successor_fn,
        eval_fn,
        usize::MAX,
        1,
        cost_fn,
        leaf_check_fn,
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
        let eval_fn = |n: &Node| n.t;

        let cost_fn = |n: &Node| n.t + time_func(n.city, start);
        let leaf_check_fn = |n: &Node| n.is_leaf();

        let (cost, best_node) =
            gds(root_node, successor_fn, eval_fn, cost_fn, leaf_check_fn).unwrap();

        assert!(cost < 9000);
        let mut visited_cities = best_node.parents.clone();
        visited_cities.push(best_node.city);
        visited_cities.sort();
        let all_cities: Vec<CityId> = (0..n_cities).into_iter().collect();
        assert_eq!(visited_cities, all_cities);
    }
}
