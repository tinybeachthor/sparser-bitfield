use std::fs::File;
use std::io;
use skiplist::skiplist::SkipList;

use crate::change::Change;
use crate::iter::Iter;

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
}

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

  /// Set a bit to true or false. Returns a boolean indicating if the value was
  /// changed.
  #[inline]
  pub fn set(&mut self, index: usize, value: bool) -> Change {
    unimplemented!();
    // let index_mask = index & 7;
    // let byte_index = (index - index_mask) / 8;
    // let byte = self.get_byte(byte_index);

    // if value {
    //   // Mask the byte to flip a bit to `1`.
    //   let byte = byte | (128 >> index_mask);
    //   self.set_byte(byte_index, byte)
    // } else {
    //   // Mask the byte to flip a bit to `0`.
    //   let byte = byte & (255 ^ (128 >> index_mask));
    //   self.set_byte(byte_index, byte)
    // }
  }

  /// Get the value of a bit.
  #[inline]
  pub fn get(&self, index: usize) -> bool {
    unimplemented!();
    // let byte_offset = index & 7;
    // let j = (index - byte_offset) / 8;

    // let num = self.get_byte(j) & (128 >> byte_offset);
    // match num {
    //   0 => false,
    //   _ => true,
    // }
  }

  /// Get a byte from our internal buffers.
  #[inline]
  pub fn get_byte(&self, index: usize) -> u8 {
    unimplemented!();
    // let byte_offset = self.page_mask(index);
    // let page_num = index / PAGE_SIZE;
    // match self.pages.get(page_num) {
    //   Some(page) => page[byte_offset],
    //   None => 0,
    // }
  }

  /// Set a byte to the right value inside our internal buffers.
  #[inline]
  pub fn set_byte(&mut self, index: usize, byte: u8) -> Change {
    unimplemented!();
    // let byte_offset = self.page_mask(index);
    // let page_num = index / PAGE_SIZE;
    // let page = self.pages.get_mut_or_alloc(page_num);

    // if index >= self.byte_length {
    //   self.byte_length = index + 1;
    // }

    // if page[byte_offset] == byte {
    //   Change::Unchanged
    // } else {
    //   page[byte_offset] = byte;
    //   Change::Changed
    // }
  }

  /// Get the amount of bits in the bitfield.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate sparser_bitfield;
  /// # use sparser_bitfield::Bitfield;
  /// let mut bits = Bitfield::new();
  /// assert_eq!(bits.len(), 0);
  /// bits.set(0, true);
  /// assert_eq!(bits.len(), 8);
  /// bits.set(1, true);
  /// assert_eq!(bits.len(), 8);
  /// bits.set(9, false);
  /// assert_eq!(bits.len(), 16);
  /// ```
  #[inline]
  pub fn len(&self) -> usize {
    unimplemented!();
    // self.byte_length * 8
  }

  /// Get the amount of bytes in the bitfield.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate sparser_bitfield;
  /// # use sparser_bitfield::Bitfield;
  /// let mut bits = Bitfield::new();
  /// assert_eq!(bits.byte_len(), 0);
  /// bits.set(0, true);
  /// assert_eq!(bits.byte_len(), 1);
  /// bits.set(1, true);
  /// assert_eq!(bits.byte_len(), 1);
  /// bits.set(9, false);
  /// assert_eq!(bits.byte_len(), 2);
  /// ```
  #[inline]
  pub fn byte_len(&self) -> usize {
    unimplemented!();
    // self.byte_length
  }

  /// Returns `true` if no bits are stored.
  ///
  /// ## Examples
  /// ```rust
  /// # extern crate sparser_bitfield;
  /// # use sparser_bitfield::Bitfield;
  /// let mut bits = Bitfield::new();
  /// assert!(bits.is_empty());
  /// bits.set(0, true);
  /// assert!(!bits.is_empty());
  /// ```
  #[inline]
  pub fn is_empty(&self) -> bool {
    unimplemented!();
    // self.pages.is_empty()
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
