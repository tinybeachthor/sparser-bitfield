use proptest::proptest;

use sparse_bitfield::{Bitfield, Change};

fn model(bit: usize) {
  let mut bits = Bitfield::new();
  assert_eq!(bits.set(bit, true), Change::Changed);
  assert_eq!(bits.get(bit), true);
}

proptest! {
  #[test]
  fn single_bit(bit in 0usize..100_000_000) {
    model(bit);
  }
}
