use std::cmp::Ordering;

use crate::store_item::Item;

/// Store text data
pub trait TextStore {
    type Item: Item;

    /// Returns the Item at [`pos`]
    fn get_at(&self, pos: usize) -> Option<&Self::Item>;

    /// Returns the amount of items stored in [`TextStore`]
    fn len(&self) -> usize;

    fn binary_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a dyn Item) -> std::cmp::Ordering;
}

impl<T: Item> TextStore for Vec<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn get_at(&self, pos: usize) -> Option<&T> {
        self.get(pos)
    }

    fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a dyn Item) -> Ordering,
    {
        let mut size = self.len();
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;

            // SAFETY: the call is made safe by the following invariants:
            // - `mid >= 0`
            // - `mid < size`: `mid` is limited by `[left; right)` bound.
            let cmp = f(unsafe { self.get_unchecked(mid) });

            // The reason why we use if/else control flow rather than match
            // is because match reorders comparison operations, which is perf sensitive.
            // This is x86 asm for u8: https://rust.godbolt.org/z/8Y8Pra.
            if cmp == Ordering::Less {
                left = mid + 1;
            } else if cmp == Ordering::Greater {
                right = mid;
            } else {
                return Ok(mid);
            }

            size = right - left;
        }
        Err(left)
    }
}
