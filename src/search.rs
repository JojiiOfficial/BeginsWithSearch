use crate::text_store::TextStore;

/// A search object. Implements the serach algorithm
pub(crate) struct Search<'a, S: TextStore> {
    store: &'a S,
    query: &'a str,
    last_pos: usize,
}

impl<'a, S: TextStore> Search<'a, S> {
    pub(crate) fn new(store: &'a S, query: &'a str) -> Self {
        Self {
            store,
            query,
            last_pos: 0,
        }
    }
}

impl<'a, S: TextStore> Iterator for Search<'a, S> {
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO implement
        None
    }
}
