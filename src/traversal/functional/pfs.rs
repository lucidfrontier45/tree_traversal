//! Priority First Search

use std::{collections::BinaryHeap, time::Duration};

use crate::utils::ScoredItem;

use super::{
    common::{NodeContainer, Reachable},
    find_best,
};

/// A container for Priority-First traversal.
pub struct PriorityFirstContainer<N, FN, FP, P: Ord> {
    to_see: BinaryHeap<ScoredItem<P, N>>,
    successor_fn: FN,
    priority_fn: FP,
}

impl<N, FN, FP, IN, P> PriorityFirstContainer<N, FN, FP, P>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FP: Fn(&N) -> Option<P>,
    P: Ord + Copy,
{
    /// Creates a new `PriorityFirstContainer` with the given successor and priority functions.
    pub fn new(start: N, successor_fn: FN, priority_fn: FP) -> Self {
        let mut to_see = BinaryHeap::new();
        if let Some(priority) = (priority_fn)(&start) {
            to_see.push(ScoredItem::from((priority, start)));
        }
        Self {
            to_see,
            successor_fn,
            priority_fn,
        }
    }
}

impl<N, FN, FP, IN, P> NodeContainer for PriorityFirstContainer<N, FN, FP, P>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FP: Fn(&N) -> Option<P>,
    P: Ord + Copy,
{
    type Node = N;

    fn pop(&mut self) -> Option<Self::Node> {
        self.to_see.pop().map(|scored| scored.into_item())
    }

    fn expand_and_push(&mut self, node: &Self::Node) {
        for s in (self.successor_fn)(node) {
            if let Some(priority) = (self.priority_fn)(&s) {
                self.to_see.push(ScoredItem::from((priority, s)));
            }
        }
    }
}

/// Creates a Priority-First Search traversal iterator starting from the given node.
///
/// This function initializes a lazy iterator that explores the tree by always selecting the
/// node with the highest priority, yielding nodes in priority-first order.
///
/// # Parameters
/// - `start`: The root node from which to begin the traversal.
/// - `successor_fn`: A function that, given a node, returns an iterator over its successor nodes.
/// - `priority_fn`: A function that evaluates a node, returning `Some(priority)` where higher priorities
///   are better, or `None` if the node cannot be evaluated (and thus skipped).
///
/// # Returns
/// An iterator that yields nodes reachable from the start node in priority-first order.
/// The iterator is lazy and will only compute successors as needed.
pub fn pfs_reach<N, IN, FN, FP, P>(
    start: N,
    successor_fn: FN,
    priority_fn: FP,
) -> Reachable<PriorityFirstContainer<N, FN, FP, P>>
where
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FP: Fn(&N) -> Option<P>,
    P: Ord + Copy,
{
    let container = PriorityFirstContainer::new(start, successor_fn, priority_fn);
    Reachable::new(container)
}

/// Find the leaf node with the lowest cost by using Priority First Search
///
/// - `start` is the start node.
/// - `successor_fn` returns a list of successors for a given node.
/// - `leaf_check_fn` check if a node is leaf or not
/// - `cost_fn` returns the final cost of a leaf node
/// - `priority_fn` returns the priority of a node, higher is better
/// - `max_ops` is the maximum number of search operations to perform
/// - `time_limit` is the maximum duration allowed for the search operation
///
/// This function returns Some of a tuple of (cost, leaf node) if found, otherwise returns None
pub fn pfs<N, IN, FN, FC, FP, C, P, FR>(
    start: N,
    successor_fn: FN,
    leaf_check_fn: FR,
    cost_fn: FC,
    priority_fn: FP,
    max_ops: usize,
    time_limit: Duration,
) -> Option<(C, N)>
where
    IN: IntoIterator<Item = N>,
    FN: FnMut(&N) -> IN,
    FC: Fn(&N) -> Option<C>,
    FP: Fn(&N) -> Option<P>,
    C: Ord + Copy + Default,
    P: Ord + Copy,
    FR: Fn(&N) -> bool,
{
    let mut res = pfs_reach(start, successor_fn, priority_fn);
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
    use std::time::Duration;

    use super::pfs;

    type Node = Vec<bool>;

    #[test]
    fn test_pfs() {
        let weights = [3, 4, 6, 5];
        let profits = [2, 3, 2, 4];
        let capacity = 8;
        let total_items = weights.len();

        let successor_fn = |n: &Node| {
            if n.len() == total_items {
                return vec![];
            }

            let total_weight: u32 = n
                .iter()
                .copied()
                .enumerate()
                .map(|(i, b)| if b { weights[i] } else { 0 })
                .sum();

            let mut children = vec![];

            let mut c1 = n.clone();
            c1.push(false);
            children.push(c1);

            let next_idx = n.len();
            if capacity >= total_weight + weights[next_idx] {
                let mut c2 = n.clone();
                c2.push(true);
                children.push(c2);
            }

            children
        };

        let cost_fn = |n: &Node| {
            let cost: u32 = n
                .iter()
                .copied()
                .enumerate()
                .map(|(i, b)| if b { profits[i] } else { 0 })
                .sum();
            Some(u32::MAX - cost)
        };

        let priority_fn = |n: &Node| {
            let current_profit: u32 = n
                .iter()
                .copied()
                .enumerate()
                .map(|(i, b)| if b { profits[i] } else { 0 })
                .sum();
            Some(current_profit)
        };

        let leaf_check_fn = |n: &Node| n.len() == total_items;
        let max_ops = usize::MAX;
        let time_limit = Duration::from_secs(10);

        let (cost, best_node) = pfs(
            vec![],
            successor_fn,
            leaf_check_fn,
            cost_fn,
            priority_fn,
            max_ops,
            time_limit,
        )
        .expect("PFS should find a valid solution");
        let cost = u32::MAX - cost;

        assert_eq!(cost, 6);
        assert_eq!(best_node, vec![true, false, false, true]);
    }
}
