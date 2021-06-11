pub mod binary_search;
pub mod store_item;
pub mod text_store;

use std::iter::from_fn;

use binary_search::Search;
use text_store::TextStore;

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
    pub fn find_all<'a>(&'a self, query: &'a str) -> Vec<&'a S::Item> {
        if query.is_empty() {
            return vec![];
        }

        Search::new(&self.text_store, query).collect()
    }

    /// Returns an iterator over all found elements
    pub fn find<'a>(&'a self, query: &'a str) -> impl Iterator<Item = &S::Item> + 'a {
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

    fn simple_ts() -> TextSearch<Vec<&'static str>> {
        TextSearch::new(vec!["a", "abc", "add", "b", "bbc"])
    }

    #[test]
    fn one_element_store() {
        let search = TextSearch::new(vec!["b"]);
        let e = search.find_all("b");
        assert_eq!(e, vec![&"b"]);
    }

    #[test]
    fn one_element_store_not_found() {
        let search = TextSearch::new(vec!["b"]);
        let e = search.find_all("0");
        let empty: Vec<&&str> = Vec::new();
        assert_eq!(e, empty);
    }

    #[test]
    fn first_matches() {
        let search = simple_ts();
        let e = search.find_all("b");
        assert_eq!(e, vec![&"b", &"bbc"]);
    }

    #[test]
    fn empty_query() {
        let search = simple_ts();
        let e = search.find_all("");
        let empty: Vec<&&str> = Vec::new();
        assert_eq!(e, empty);
    }

    #[test]
    fn not_found() {
        let search = simple_ts();
        let e = search.find_all("0");
        let empty: Vec<&&str> = Vec::new();
        assert_eq!(e, empty);
    }

    /*
    #[test]
    fn test_2st_matches() {
        let search = simple_ts();
        let e = search.find_all("bb");
        assert_eq!(e, vec![&"bbc"]);
    }
    */
}
