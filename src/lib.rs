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

    fn simple_ts2() -> TextSearch<Vec<&'static str>> {
        TextSearch::new(vec!["b", "bbc"])
    }

    fn simple_dataset() -> Vec<TextSearch<Vec<&'static str>>> {
        vec![simple_ts(), simple_ts2()]
        //vec![simple_ts2()]
    }

    #[test]
    fn first_matches() {
        for search in simple_dataset() {
            let e = search.find_all("b");
            assert_eq!(e, vec![&"b", &"bbc"]);
        }
    }

    #[test]
    fn empty_query() {
        for search in simple_dataset() {
            let e = search.find_all("");
            let empty: Vec<&&str> = Vec::new();
            assert_eq!(e, empty);
        }
    }

    #[test]
    fn not_found() {
        for search in simple_dataset() {
            let e = search.find_all("0");
            let empty: Vec<&&str> = Vec::new();
            assert_eq!(e, empty);
        }
    }

    #[test]
    fn test_2st_matches() {
        for search in simple_dataset() {
            let e = search.find_all("bb");
            assert_eq!(e, vec![&"bbc"]);
        }
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
    fn test_matches_complex() {
        let search = bigger_ts();
        let e = search.find_all("go");
        assert_eq!(e, vec![&"go", &"golang"]);
    }

    #[test]
    fn test_matches_complex2() {
        let search = bigger_ts();
        let e = search.find_all("ga");
        let empty: Vec<&&str> = Vec::new();
        assert_eq!(e, empty);
    }

    fn bigger_ts() -> TextSearch<Vec<&'static str>> {
        let input = vec!["a", "b", "go", "golang", "rust"];
        let mut input_sorted = vec!["a", "b", "go", "golang", "rust"];
        input_sorted.sort_unstable();
        assert_eq!(input, input_sorted);
        TextSearch::new(input_sorted)
    }
}
