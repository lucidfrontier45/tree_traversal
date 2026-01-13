/// A small wrapper combining a score and an associated item.
///
/// `ScoredItem` is used internally by traversal algorithms to store items in
/// priority queues where ordering is based on the score. The score type `S`
/// must implement `Ord` so items can be compared.
pub struct ScoredItem<S: Ord, A>(S, A);

impl<S: Ord, A> ScoredItem<S, A> {
    /// Consumes the scored item and returns the inner associated value, discarding the score.
    pub fn into_item(self) -> A {
        self.1
    }

    /// Consumes the scored item and returns the score and the associated value.
    pub fn into_inner(self) -> (S, A) {
        (self.0, self.1)
    }
}

impl<S: Ord, A> PartialEq for ScoredItem<S, A> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S: Ord, A> Eq for ScoredItem<S, A> {}

impl<S: Ord, A> PartialOrd for ScoredItem<S, A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: Ord, A> Ord for ScoredItem<S, A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<S: Ord, A> From<(S, A)> for ScoredItem<S, A> {
    fn from(v: (S, A)) -> Self {
        let (cost, item) = v;
        ScoredItem(cost, item)
    }
}
