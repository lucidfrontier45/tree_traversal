pub struct ScoredItem<S: Ord, A>(S, A);

impl<S: Ord, A> ScoredItem<S, A> {
    pub fn into_item(self) -> A {
        self.1
    }

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
