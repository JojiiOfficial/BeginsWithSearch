mod search;
pub mod store_item;
pub mod text_store;

use search::Search;
use text_store::TextStore;

pub struct TextSearch<S: TextStore> {
    text_store: S,
}

impl<S: TextStore> TextSearch<S> {
    /// Creates a new [`Serach`] based on searchable data. The input must be sorted and implement
    /// `Ord`
    pub fn new(text_store: S) -> Self {
        Self { text_store }
    }

    /// Returns a vector over all found elements
    pub fn find_all<'a>(&'a self, query: &'a str) -> Vec<&'a S::Item> {
        if query.is_empty() {
            return vec![];
        }

        let search = self.search(query);
        search.search().collect()
    }

    /// Same as `find_all` but returns an iterator over each element
    pub fn find<'a>(&'a self, query: &'a str) -> impl Iterator<Item = &'a S::Item> {
        self.search(query).search()
    }

    fn search<'a>(&'a self, query: &'a str) -> Search<'a, S> {
        Search::new(query, &self.text_store)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Write},
        time::SystemTime,
    };

    use simsearch::{SearchOptions, SimSearch};

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

    #[test]
    fn file() {
        let file = File::open("./output.json").unwrap();
        let reader = BufReader::new(file);
        let vec: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();

        let searc = TextSearch::new(vec);
        let start = SystemTime::now();
        let res = searc.find_all("music");
        println!("found: {}", res.len());
        println!("took {:?}", start.elapsed());
    }

    #[test]
    fn file_sidm() {
        let file = File::open("./output.json").unwrap();
        let reader = BufReader::new(file);
        let vec: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();

        let options = SearchOptions::new()
            .levenshtein(true)
            .threshold(0.99)
            .case_sensitive(true);
        let mut engine: SimSearch<String> = SimSearch::new_with(options);
        for (i, v) in vec.into_iter().enumerate() {
            engine.insert(v.clone(), &v);
        }
        let start = SystemTime::now();
        let results: Vec<_> = engine.search("music");
        println!("sdim found: {:#?}", results);
        println!("sdim took {:?}", start.elapsed());
        println!("sdim found: {}", results.len());
    }
}
