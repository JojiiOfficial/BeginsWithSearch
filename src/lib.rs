pub mod store_item;
pub mod text_store;

use std::{cmp::Ordering, iter::from_fn};

use store_item::Item;
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

        self.binary_search_custom(query)
    }

    fn binary_search_custom<'a>(&'a self, query: &str) -> Vec<&'a S::Item> {
        let bin_search = self
            .text_store
            .binary_search_by(|a| my_cmp(a.get_text(), &query));
        let index = match bin_search {
            Ok(s) => s,
            Err(s) => s,
        };

        if index >= self.text_store.len() {
            return vec![];
        }

        let mut pos = index;
        loop {
            if pos == 0 {
                break;
            }
            let prev_pos = pos - 1;
            let prev_item = self.text_store.get_at(prev_pos).unwrap();
            if my_cmp(prev_item.get_text(), query) == Ordering::Equal {
                if pos == 0 {
                    break;
                }
                pos = prev_pos;
            } else {
                break;
            }
        }
        let first_item = pos;

        let mut res = Vec::new();

        for pos in first_item..self.text_store.len() {
            let item = self.text_store.get_at(pos).unwrap();
            if my_cmp(item.get_text(), query) == Ordering::Equal {
                res.push(item);
            } else {
                break;
            }
        }

        res
    }
}

fn my_cmp(a: &str, b: &str) -> Ordering {
    if a.starts_with(b) {
        Ordering::Equal
    } else {
        a.cmp(b)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Write},
    };

    use super::*;

    fn simple_ts() -> TextSearch<Vec<&'static str>> {
        TextSearch::new(vec!["a", "abc", "add", "b", "bbc"])
    }

    fn simple_ts2() -> TextSearch<Vec<&'static str>> {
        TextSearch::new(vec!["b", "bbc"])
    }

    fn bigger_ts() -> TextSearch<Vec<&'static str>> {
        let input = vec!["a", "b", "go", "golang", "rust"];
        let mut input_sorted = vec!["a", "b", "go", "golang", "rust"];
        input_sorted.sort_unstable();
        assert_eq!(input, input_sorted);
        TextSearch::new(input_sorted)
    }

    fn simple_dataset() -> Vec<TextSearch<Vec<&'static str>>> {
        vec![simple_ts(), simple_ts2()]
    }

    #[test]
    fn first_matches() {
        for search in simple_dataset() {
            let e = search.find_all("b");
            assert_eq!(e, vec![&"b", &"bbc"]);
        }
    }

    #[test]
    fn one_element_store() {
        let search = TextSearch::new(vec!["b"]);
        let e = search.find_all("b");
        assert_eq!(e, vec![&"b"]);
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
}
