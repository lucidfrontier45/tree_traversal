//! Common functional utilities for tree traversal algorithms.

use std::{collections::BinaryHeap, iter::FusedIterator, time::Duration};

use crate::utils::ScoredItem;

/// Performs a generic traversal over a tree iterator, collecting the best leaf nodes based on their costs.
///
/// This function iterates through the provided tree, checks each node to determine if it's a leaf using the `leaf_check_fn`,
/// computes its cost using the `cost_fn`, and maintains a priority queue of the top `queue_size` nodes with the lowest costs.
/// The traversal stops early if the maximum number of operations (`max_ops`) is reached or the time limit is exceeded.
///
/// The `callback_fn` is invoked for every visited node with its index and a reference to the node. It can be used for
/// progress reporting, logging, or other side-effects. The callback is called before leaf/cost checks.
///
/// # Parameters
/// - `tree`: A mutable reference to a fused iterator over the tree nodes.
/// - `leaf_check_fn`: A function that checks if a node is a leaf.
/// - `cost_fn`: A function that computes the cost of a node, returning `None` if the cost cannot be determined.
/// - `max_ops`: The maximum number of nodes to process.
/// - `time_limit`: The maximum time allowed for the traversal.
/// - `queue_size`: The maximum number of best nodes to keep in the result.
/// - `callback_fn`: A mutable callback invoked as `callback_fn(n_step, &node)` for each visited node.
///
/// # Returns
/// A vector of tuples containing the cost and the node, limited to `queue_size`.
pub fn traverse<C, N, FC, FL, CB>(
    tree: &mut impl FusedIterator<Item = N>,
    leaf_check_fn: FL,
    cost_fn: FC,
    max_ops: usize,
    time_limit: Duration,
    queue_size: usize,
    mut callback_fn: CB,
) -> Vec<(C, N)>
where
    C: Ord + Copy,
    FC: Fn(&N) -> Option<C>,
    FL: Fn(&N) -> bool,
    CB: FnMut(usize, &N),
{
    let mut queue = BinaryHeap::new();

    let start = std::time::Instant::now();
    for (i, n) in tree.enumerate() {
        if i >= max_ops || start.elapsed() >= time_limit {
            break;
        }
        callback_fn(i, &n);

        if !leaf_check_fn(&n) {
            continue;
        }

        let Some(cost) = cost_fn(&n) else {
            continue;
        };

        queue.push(ScoredItem::from((cost, n)));
        if queue.len() > queue_size {
            queue.pop();
        }
    }

    queue
        .into_iter()
        .take(queue_size)
        .map(|item| {
            let (cost, n) = item.into_inner();
            (cost, n)
        })
        .collect()
}

/// Finds the best (lowest cost) leaf node in the tree iterator within the given constraints.
///
/// This function is a convenience wrapper around `traverse` that returns only the single best node.
/// It accepts the same parameters as `traverse`, including `callback_fn`, which is invoked for each
/// visited node. Use the callback for logging, progress updates, or to collect statistics about visited nodes.
///
/// # Parameters
/// - `tree`: A mutable reference to a fused iterator over the tree nodes.
/// - `leaf_check_fn`: A function that checks if a node is a leaf.
/// - `cost_fn`: A function that computes the cost of a node, returning `None` if the cost cannot be determined.
/// - `max_ops`: The maximum number of nodes to process.
/// - `time_limit`: The maximum time allowed for the traversal.
/// - `callback_fn`: A mutable callback invoked as `callback_fn(n_step, &node)` for each visited node.
///
/// # Returns
/// The best (lowest cost) leaf node and its cost, or `None` if no valid leaf is found.
pub fn find_best<C, N, FC, FL, CB>(
    tree: &mut impl FusedIterator<Item = N>,
    leaf_check_fn: FL,
    cost_fn: FC,
    max_ops: usize,
    time_limit: Duration,
    mut callback_fn: CB,
) -> Option<(C, N)>
where
    C: Ord + Copy,
    FC: Fn(&N) -> Option<C>,
    FL: Fn(&N) -> bool,
    CB: FnMut(usize, &N),
{
    traverse(
        tree,
        leaf_check_fn,
        cost_fn,
        max_ops,
        time_limit,
        1, // only need the best one
        &mut callback_fn,
    )
    .pop()
}

/// A trait representing a container that holds the frontier of nodes during a traversal.
///
/// This trait is the core abstraction used by traversal drivers such as [`Reachable`].
/// It decouples *how* nodes are stored and scheduled for visiting (stack, queue, priority
/// queue, etc.) from the traversal logic itself.
///
/// # Traversal contract
///
/// A typical traversal loop using a `NodeContainer` behaves as follows:
///
/// 1. Repeatedly call [`pop`](NodeContainer::pop) to obtain the next node to visit.
/// 2. For each node returned as `Some(node)`, call
///    [`expand_and_push`](NodeContainer::expand_and_push) with a reference to that node.
///    This method is responsible for discovering the node's children and inserting them
///    into the container.
/// 3. When [`pop`](NodeContainer::pop) returns `None`, the container is empty and the
///    traversal is finished.
///
/// The [`Reachable`] iterator follows exactly this protocol in its [`Iterator::next`]
/// implementation: it calls `pop()`, then (if successful) immediately calls
/// `expand_and_push(&node)` with the popped node.
///
/// # Traversal order
///
/// The concrete `NodeContainer` implementation fully determines the traversal order:
///
/// - A stack-like container (LIFO) results in a depth-first traversal.
/// - A queue-like container (FIFO) results in a breadth-first traversal.
/// - A priority-based container can implement best-first, Dijkstra-like, or A*-like
///   traversals, depending on the priority policy.
///
/// Implementations are expected to store nodes (and any associated metadata they need)
/// internally and to be safe for repeated `pop` / `expand_and_push` cycles as described
/// above.
pub trait NodeContainer {
    /// The type of nodes stored in the container.
    type Node;

    /// Removes and returns the next node to be visited from the container.
    ///
    /// This method is called by traversal drivers (such as [`Reachable`]) to obtain the
    /// next node in the traversal. If it returns `Some(node)`, the driver will then call
    /// [`expand_and_push`](NodeContainer::expand_and_push) with a reference to that same
    /// node before visiting the next one.
    ///
    /// Returning `None` indicates that there are no more nodes to visit and that the
    /// traversal is complete.
    fn pop(&mut self) -> Option<Self::Node>;

    /// Expands the given node and inserts its children into the container.
    ///
    /// This method is called once for each node immediately after it has been obtained
    /// via [`pop`](NodeContainer::pop). Implementations should use the provided `node`
    /// to determine which nodes are reachable from it and push those children into the
    /// container according to their scheduling policy (e.g., LIFO, FIFO, priority).
    ///
    /// The `node` itself must not be consumed or modified by this method; it is passed
    /// by shared reference so that it can also be yielded to the caller of the traversal.
    fn expand_and_push(&mut self, node: &Self::Node);
}

/// A Generic iterator that traverses nodes using a specified node container.
pub struct Reachable<C> {
    to_see: C,
}

impl<C> Reachable<C> {
    /// Creates a new `Reachable` iterator with the given node container.
    pub fn new(to_see: C) -> Self {
        Self { to_see }
    }
}

impl<N, C> Iterator for Reachable<C>
where
    C: NodeContainer<Node = N>,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.to_see.pop()?;
        self.to_see.expand_and_push(&node);
        Some(node)
    }
}

impl<C> FusedIterator for Reachable<C> where C: NodeContainer {}
