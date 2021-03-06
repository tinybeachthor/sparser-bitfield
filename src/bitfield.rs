use skiplist::skiplist::SkipList;

use crate::change::Change;

pub type Range = std::ops::Range<usize>;

/// Bitfield instance.
#[derive(Debug)]
pub struct Bitfield {
    /// A [skiplist] instance.
    ///
    /// [skiplist]: https://docs.rs/skiplist/
    skiplist: SkipList<Range>,
}

impl Bitfield {
    /// Create a new instance.
    ///
    /// ## Panics
    /// The page size must be a multiple of 2, and bigger than 0.
    #[inline]
    pub fn new() -> Self {
        Bitfield {
            skiplist: SkipList::new(),
        }
    }

    /// Set or reset a bit.
    /// Returns a `Change` indicating if the value was changed.
    #[inline]
    pub fn set_or_reset(&mut self, index: usize, value: bool) -> Change {
        if value {
            self.set(index)
        }
        else {
            self.reset(index)
        }
    }

    /// Set a bit.
    /// Returns a `Change` indicating if the value was changed.
    #[inline]
    pub fn set(&mut self, index: usize) -> Change {
        let mut iter = self.skiplist
            .iter_mut()
            // advance iterator to the relevant part
            .skip_while(|range| range.start < index);

        match iter.next() {
            // inserting at end
            None => match self.skiplist.back_mut() {
                // skiplist is empty
                None => {
                    self.skiplist
                        .push_back(Range { start: index, end: index + 1 });
                    Change::Changed
                },
                // skiplist is not empty
                Some(range) =>
                    // inside the range
                    if range.contains(&index) {
                        Change::Unchanged
                    }
                    // at the end of the range
                    else if range.end == index {
                        range.end = range.end + 1;
                        Change::Changed
                    }
                    // after the range
                    else {
                        self.skiplist
                            .push_back(Range { start: index, end: index + 1});
                        Change::Changed
                    },
            },
            // inserting in middle or beginning (not at end)
            Some(range) => {
                // inside the range
                if range.contains(&index) {
                    Change::Unchanged
                }
                // at the end of the range
                else if range.end == index {
                    range.end = range.end + 1;

                    // check if we can merge with next range
                    if let Some(next) = iter.next() {
                        if range.end == next.start {
                            range.end = next.end;
                            if let Some(next_index) = iter.position(|_| true) {
                                self.skiplist
                                    .remove(next_index - 1);
                            }
                        }
                    }

                    Change::Changed
                }
                // after the range
                else {
                    match iter.next() {
                        // insert at end
                        None => {
                            self.skiplist
                                .push_back(Range { start: index, end: index + 1 });
                            Change::Changed
                        },
                        // inserting before a next range
                        Some(next) => {
                            if index + 1 == next.start {
                                next.start = index
                            }
                            else {
                                let new_range = Range { start: index, end: index + 1 };

                                match iter.position(|_| true) {
                                    Some(next_index) => {
                                        if next_index > 0 {
                                            self.skiplist.insert(new_range, next_index - 1);
                                        }
                                        else {
                                            self.skiplist.push_front(new_range);
                                        }
                                    },
                                    None => {
                                        self.skiplist.push_back(new_range);
                                    },
                                }
                            }
                            Change::Changed
                        },
                    }
                }
            },
        }
    }

    /// Reset a bit.
    /// Returns a `Change` indicating if the value was changed.
    #[inline]
    pub fn reset(&mut self, _index: usize) -> Change {
        unimplemented!()
    }

    /// Get the value of a bit.
    #[inline]
    pub fn get(&self, index: usize) -> bool {
        let mut iter = self.skiplist
            .iter()
            .skip_while(|range| {
                index < range.start
            });

        match iter.next() {
            None => false,
            Some(range) => range.contains(&index),
        }
    }

    /// Get the amount of bits in the bitfield.
    ///
    /// ## Examples
    /// ```rust
    /// # extern crate sparser_bitfield;
    /// # use sparser_bitfield::Bitfield;
    /// let mut bits = Bitfield::new();
    /// assert_eq!(bits.len(), 0);
    /// bits.set(0);
    /// assert_eq!(bits.len(), 1);
    /// bits.set(1);
    /// assert_eq!(bits.len(), 2);
    /// bits.reset(9);
    /// assert_eq!(bits.len(), 2);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.skiplist
            .back()
            .map_or(
                0,
                |last| last.end)
    }

    /// Returns `true` if no bits are stored.
    ///
    /// ## Examples
    /// ```rust
    /// # extern crate sparser_bitfield;
    /// # use sparser_bitfield::Bitfield;
    /// let mut bits = Bitfield::new();
    /// assert!(bits.is_empty());
    /// bits.set(0);
    /// assert!(!bits.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.skiplist
            .is_empty()
    }
}
