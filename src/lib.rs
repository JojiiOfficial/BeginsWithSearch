use std::iter::from_fn;

use text_store::TextStore;

use crate::search::Search;

pub mod search;
pub mod store_item;
pub mod text_store;

pub struct TextSearch<S: TextStore> {
    text_store: S,
}

impl<S: TextStore> TextSearch<S> {
    /// Creates a new [`Serach`] based on searchable data. The input must be sorted and implement
    /// `PartialEq`
    pub fn new(text_store: S) -> Self {
        Self { text_store }
    }

    /// Returns a vector over all found elements
    pub fn find_all(&self, query: &str) -> Vec<S::Item> {
        if query.is_empty() {
            return vec![];
        }

        Search::new(&self.text_store, query).collect()
    }

    /// Returns an iterator over all found elements
    pub fn find<'a>(&'a self, query: &'a str) -> impl Iterator<Item = S::Item> + 'a {
        let mut search = Search::new(&self.text_store, query);
        from_fn(move || {
            if query.is_empty() {
                None
            } else {
                search.next()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let search = TextSearch::new(vec!["a", "b", "abc", "bbc"]);
        let e = search.find_all("b");
        assert_eq!(e, vec!["b", "bbc"]);
    }
}
