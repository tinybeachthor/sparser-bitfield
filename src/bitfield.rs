use std::fs::File;
use std::io;
use skiplist::skiplist::SkipList;

use crate::change::Change;
use crate::iter::Iter;

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

  /// Create a new instance from a `File`.
  pub fn from_file(
    file: &mut File,
    offset: Option<usize>,
  ) -> io::Result<Self> {
    unimplemented!();
    // let pages = Pager::from_file(file, PAGE_SIZE, offset)?;

    // // NOTE: empty pages are initialized as `0` filled. So when we reinitialize
    // // a page, in essence our byte length becomes the amount of bytes we have
    // // times the amount of pages we have.
    // let byte_length = pages.len() * PAGE_SIZE;

    // Ok(Self {
    //   pages,
    //   byte_length,
    // })
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
                            if let Some(index) = iter.position(|_| true) {
                                self.skiplist
                                    .remove(index - 1);
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
                                if let Some(index) = iter.position(|_| true) {
                                    self.skiplist
                                        .insert(Range { start: index, end: index + 1 },
                                                index - 1);
                                }
                                else {
                                    unreachable!();
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
    pub fn reset(&mut self, index: usize) -> Change {
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
    /// assert_eq!(bits.len(), 8);
    /// bits.set(1);
    /// assert_eq!(bits.len(), 8);
    /// bits.reset(9);
    /// assert_eq!(bits.len(), 16);
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

  /// Create an `Iterator` that iterates over all pages.
  #[inline]
  pub fn iter(&mut self) -> Iter {
    unimplemented!();
    // Iter::new(self)
  }

  /// Based on [Bitfield.prototype.toBuffer].
  ///
  /// [Bitfield.prototype.toBuffer]: https://github.com/mafintosh/sparse-bitfield/blob/master/index.js#L54-L64
  pub fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
    unimplemented!();
    // use std::io::{Cursor, Write};

    // let mut all =
    //   Cursor::new(Vec::with_capacity(self.pages.len() * PAGE_SIZE));

    // for index in 0..PAGE_SIZE {
    //   let next = self.pages.get(index);
    //   if let Some(page) = next {
    //     let all_offset = index * PAGE_SIZE;
    //     all.set_position(all_offset as u64);
    //     all.write_all(&page)?;
    //   }
    // }

    // Ok(all.into_inner())
  }
}
