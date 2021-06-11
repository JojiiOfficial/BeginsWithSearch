/// Store text data
pub trait TextStore {
    type Item: Ord + Sized;

    /// Returns the Item at [`pos`]
    fn get_at(&self, pos: usize) -> Option<&Self::Item>;

    /// Returns the amount of items stored in [`TextStore`]
    fn len(&self) -> usize;
}

impl<T: Ord> TextStore for Vec<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn get_at(&self, pos: usize) -> Option<&T> {
        self.get(pos)
    }
}
