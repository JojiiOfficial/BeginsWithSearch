use std::cmp::Ordering;

use crate::store_item::Item;
use crate::text_store::TextStore;

/// A search object. Implements the binary search algorithm for substring matching. Implement
/// `Iterator` which iterates over the results
pub(crate) struct Search<'a, S: TextStore> {
    store: &'a S,
    query: &'a str,
    last_pos: usize,
    last_str_index: usize,
    found_first: bool,
}

impl<'a, S: TextStore> Search<'a, S> {
    /// Create a new search item. Allows to be used as iterator over results
    pub(crate) fn new(store: &'a S, query: &'a str) -> Self {
        Self {
            store,
            query,
            last_pos: store.len(),
            last_str_index: 0,
            found_first: false,
        }
    }

    /// Checks if the search input is valid
    fn is_valid(&self) -> bool {
        let store_len = self.store.len();
        let query = self.query;
        store_len != 0 && !query.is_empty()
    }

    /// Compares the item with the query
    fn compare(&self, i: &S::Item) -> Ordering {
        let i = i.get_text();

        if self.query.len() > i.len() {
            //return Ordering::Less;
        }

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
            // End  reached
            if self.last_pos == 0 && !self.found_first {
                break None;
            }

            let curr_index = if self.found_first {
                // Item found, we only want to iterate from here on
                self.last_pos + 1
            } else {
                // Continue binary search
                self.last_pos / 2
            };

            let curr_item = self.store.get_at(curr_index)?;

            match self.compare(curr_item) {
                Ordering::Equal => {
                    // We want to be able to match the entire substring, not only the first
                    // character
                    if self.last_str_index < self.query.len() - 1 {
                        self.last_str_index += 1;
                        continue;
                    }

                    let mut curr_item = curr_item;

                    if !self.found_first {
                        self.found_first = true;
                        self.last_pos = curr_index;

                        // Find first position
                        for i in (0..curr_index).rev() {
                            let citem = self.store.get_at(i)?;
                            if self.compare(citem) == Ordering::Equal {
                                curr_item = citem;
                                self.last_pos = i;
                            } else {
                                break;
                            }
                        }
                    } else {
                        self.last_pos = curr_index;
                    }

                    // Skip entries with shorter text as query
                    if curr_item.get_text().len() < self.query.len() {
                        continue;
                    }

                    // Item found
                    return Some(curr_item);
                }
                Ordering::Greater => {
                    // Jump to bigger half
                    self.last_pos = curr_index + self.last_pos;
                }
                Ordering::Less => {
                    // Items already found. This indicates the end of all matching items since the
                    // input is sorted
                    if self.found_first {
                        return None;
                    }

                    // Jump to lower half
                    self.last_pos = curr_index;
                }
            }
        }
    }
}
