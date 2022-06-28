/// A push only vector that insures all elements are unique.
/// Used as a wrapper around Vec<T> for text styles.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UniqueVec<T> {
    inner: Vec<T>,
}

impl<T> UniqueVec<T>
where
    T: Ord,
{
    /// Return a new UniqueVec<T> with an empty inner Vec<T>.
    pub(crate) fn new() -> UniqueVec<T> {
        UniqueVec { inner: Vec::new() }
    }

    /// Exposed push method for UniqueVec<T>. Insures all pushed
    /// elements are unique.
    pub(crate) fn push(&mut self, value: T) {
        self.inner.push(value);
        self.make_unique();
    }

    /// Sort and then remove all duplicates. Called after a push method.
    fn make_unique(&mut self) {
        self.inner.sort();
        self.inner.dedup();
    }
}

impl<T> IntoIterator for UniqueVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Return the wrapped vector as an iterator.
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
