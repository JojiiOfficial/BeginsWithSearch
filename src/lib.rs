mod binary_search;
mod jaro_search;
pub mod store_item;
pub mod text_store;

use binary_search::Search as BinarySearch;
use jaro_search::Search as JaroSearch;
use text_store::TextStore;

pub struct TextSearch<'a, T: TextStore> {
    text_store: &'a T,
}

impl<'a, T: TextStore> TextSearch<'a, T> {
    /// Creates a new [`Serach`] based on searchable data. The input must be sorted and implement
    /// `Ord`
    pub fn new(text_store: &'a T) -> Self {
        Self { text_store }
    }

    /// Returns a vector over all found elements
    pub fn find_all_bin(&'a self, query: &'a str) -> Vec<&'a T::Item> {
        if query.is_empty() {
            return vec![];
        }

        self.find_binary(query).collect()
    }

    /// Returns a vector over all found elements
    pub fn find_all_lev(&'a self, query: &'a str, len_limit: usize) -> Vec<&'a T::Item> {
        if query.is_empty() {
            return vec![];
        }

        self.find_jaro(query, len_limit).collect()
    }

    pub fn find_jaro(
        &'a self,
        query: &'a str,
        len_limit: usize,
    ) -> impl Iterator<Item = &'a T::Item> {
        self.jaro_search(query, len_limit).search()
    }

    pub fn find_binary(&'a self, query: &'a str) -> impl Iterator<Item = &'a T::Item> {
        self.binary_search(query).search()
    }

    fn binary_search(&'a self, query: &'a str) -> BinarySearch<'a, T> {
        BinarySearch::new(query, &self.text_store)
    }

    fn jaro_search(&'a self, query: &'a str, len_limit: usize) -> JaroSearch<'a, T> {
        JaroSearch::new(query, &self.text_store, len_limit)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Write},
        time::SystemTime,
    };

    use strsim::jaro_winkler;

    use super::*;

    const TS: &'static [&'static str] = &["a", "abc", "add", "b", "bbc"];
    const TS2: &'static [&'static str] = &["b", "bbc"];
    const B_TS: &'static [&'static str] = &["a", "b", "go", "golang", "rust"];

    fn simple_ts() -> TextSearch<'static, &'static [&'static str]> {
        TextSearch::new(&TS)
    }

    fn simple_ts2() -> TextSearch<'static, &'static [&'static str]> {
        TextSearch::new(&TS2)
    }

    fn bigger_ts() -> TextSearch<'static, &'static [&'static str]> {
        TextSearch::new(&B_TS)
    }

    fn simple_dataset() -> Vec<TextSearch<'static, &'static [&'static str]>> {
        vec![simple_ts(), simple_ts2()]
    }

    #[test]
    fn first_matches() {
        for search in simple_dataset() {
            let e = search.find_all_bin("b");
            assert_eq!(e, vec![&"b", &"bbc"]);
        }
    }

    #[test]
    fn one_element_store() {
        let data = vec!["b"];
        let search = TextSearch::new(&data);
        let e = search.find_all_bin("b");
        assert_eq!(e, vec![&"b"]);
    }

    #[test]
    fn empty_query() {
        for search in simple_dataset() {
            let e = search.find_all_bin("");
            let empty: Vec<&&str> = Vec::new();
            assert_eq!(e, empty);
        }
    }

    #[test]
    fn not_found() {
        for search in simple_dataset() {
            let e = search.find_all_bin("0");
            let empty: Vec<&&str> = Vec::new();
            assert_eq!(e, empty);
        }
    }

    #[test]
    fn test_2st_matches() {
        for search in simple_dataset() {
            let e = search.find_all_bin("bb");
            assert_eq!(e, vec![&"bbc"]);
        }
    }

    #[test]
    fn one_element_store_not_found() {
        let data = vec!["b"];
        let search = TextSearch::new(&data);
        let e = search.find_all_bin("0");
        let empty: Vec<&&str> = Vec::new();
        assert_eq!(e, empty);
    }

    #[test]
    fn test_matches_complex() {
        let search = bigger_ts();
        let e = search.find_all_bin("go");
        assert_eq!(e, vec![&"go", &"golang"]);
    }

    #[test]
    fn test_matches_complex2() {
        let search = bigger_ts();
        let e = search.find_all_bin("ga");
        let empty: Vec<&&str> = Vec::new();
        assert_eq!(e, empty);
    }

    #[test]
    fn file() {
        let file = File::open("./output_en.json").unwrap();
        let reader = BufReader::new(file);
        let vec: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();

        let searc = TextSearch::new(&vec);
        let start = SystemTime::now();
        let res = searc.find_all_bin("music");
        println!("binary found: {}", res.len());
        println!("binary took {:?}", start.elapsed());
    }

    #[test]
    fn file_lev() {
        let file = File::open("./output_en.json").unwrap();
        let reader = BufReader::new(file);
        let vec: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();

        let searc = TextSearch::new(&vec);
        let start = SystemTime::now();
        let query = "cmon";
        let mut res: Vec<&String> = searc.find_jaro(query, 5).collect();
        res.sort_by(|l, r| {
            let l_j = (jaro_winkler(l, query) * 100_f64) as u32;
            let r_j = (jaro_winkler(r, query) * 100_f64) as u32;
            r_j.cmp(&l_j)
        });

        println!("{:#?}", res.iter().take(10000).collect::<Vec<_>>());
        println!("lev found: {}", res.len());
        println!("lev took {:?}", start.elapsed());
    }
}
