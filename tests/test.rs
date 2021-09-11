use sparser_bitfield::{Bitfield, Change};

#[test]
fn can_create_bitfield() {
  let _bits = Bitfield::new();
}

#[test]
fn basic_set_get() {
  let mut bits = Bitfield::new();
  bits.set(0);
  assert_eq!(
      bits.get(0),
      true);
}

#[test]
fn can_set_bits() {
  let mut bits = Bitfield::new();
  bits.set(100);
  bits.set(1_000);
  bits.set(1_000_000);
  bits.set(1_000_000_000);
  bits.set(1_000_000_000_000);
}

#[test]
fn can_get_bits() {
  let mut bits = Bitfield::new();
  bits.set(0);
  bits.set(1);
  bits.set(1000);
  assert_eq!(bits.get(0), true);
  assert_eq!(bits.get(1), true);
}

#[test]
fn returns_if_flipped() {
  let mut bits = Bitfield::new();
  assert_eq!(bits.set(0), Change::Changed);
  assert_eq!(bits.reset(0), Change::Changed);
  assert_eq!(bits.set(0), Change::Changed);
  assert_eq!(bits.set(0), Change::Unchanged);
  assert_eq!(bits.set(0), Change::Unchanged);
}

#[test]
fn exposes_changed_unchanged_methods() {
  let mut bits = Bitfield::new();
  assert!(bits.set(0).is_changed());
  assert!(bits.set(0).is_unchanged());
}
