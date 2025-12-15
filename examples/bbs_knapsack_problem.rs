use std::time::Duration;

use tree_traversal::functional::bbs;

type Node = Vec<bool>;
fn main() {
    let weights = [4, 2, 6, 3, 4];
    let profits = [100, 20, 2, 5, 10];
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
            .map(|(i, b)| if b { profits[i] } else { 0 })
            .sum();
        s
    };

    let lower_bound_fn = |n: &Node| {
        let current_profit = total_profit(n);
        let max_remained_profit: u32 = profits[n.len()..].iter().sum();
        Some(u32::MAX - (current_profit + max_remained_profit))
    };

    let cost_fn = |n: &Node| Some(u32::MAX - total_profit(n));

    let leaf_check_fn = |n: &Node| n.len() == total_items;
    let max_ops = usize::MAX;
    let time_limit = Duration::from_secs(10);

    let (cost, best_node) = bbs(
        vec![],
        successor_fn,
        leaf_check_fn,
        cost_fn,
        lower_bound_fn,
        max_ops,
        time_limit,
    )
    .unwrap();
    let cost = u32::MAX - cost;

    dbg!((best_node, cost));
}
