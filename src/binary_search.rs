use std::cmp::Ordering;

use crate::store_item::Item;
use crate::text_store::TextStore;

/// A search object. Implements the serach algorithm
pub(crate) struct Search<'a, S: TextStore> {
    store: &'a S,
    query: &'a str,
    last_pos: usize,
    last_str_index: usize,
    found_first: bool,
}

impl<'a, S: TextStore> Search<'a, S> {
    pub(crate) fn new(store: &'a S, query: &'a str) -> Self {
        Self {
            store,
            query,
            last_pos: store.len(),
            last_str_index: 0,
            found_first: false,
        }
    }

    fn is_valid(&self) -> bool {
        let store_len = self.store.len();
        let query = self.query;
        store_len != 0 && !query.is_empty()
    }

    fn compare(&self, i: &str) -> Ordering {
        let item_char = i
            .chars()
            .nth(self.last_str_index)
            .unwrap_or(i.chars().next().unwrap());

        let query_char = self.query.chars().nth(self.last_str_index).unwrap();
        query_char.cmp(&item_char)
    }
}

impl<'a, S: TextStore> Iterator for Search<'a, S> {
    type Item = &'a S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.is_valid().then(|| 0)?;

        loop {
            if self.last_pos == 0 {
                break None;
            }

            let curr_index = if self.found_first {
                self.last_pos + 1
            } else {
                self.last_pos / 2
            };

            let curr_item = self.store.get_at(curr_index)?;

            match self.compare(curr_item.get_text()) {
                Ordering::Equal => {
                    println!("equal: {}", curr_item.get_text());
                    self.last_pos = curr_index;
                    self.found_first = true;
                    return Some(curr_item);
                }
                Ordering::Greater => {
                    self.last_pos = curr_index + self.last_pos;
                }
                Ordering::Less => {
                    if self.found_first {
                        return None;
                    }
                    self.last_pos = curr_index;
                }
            }
        }
    }
}
