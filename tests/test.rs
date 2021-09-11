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
  // bits.set(1_000_000_000_000, true);
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

#[test]
fn can_iterate() {
  let mut bits = Bitfield::new();

  bits.set(0);
  for (i, bit) in bits.iter().enumerate() {
    match i {
      0 => assert_eq!(bit, true),
      _ => assert_eq!(bit, false),
    }
  }

  let arr: Vec<bool> = bits.iter().collect();
  assert_eq!(arr.len(), 8);

  bits.reset(1);
  for (i, bit) in bits.iter().enumerate() {
    match i {
      0 => assert_eq!(bit, true),
      _ => assert_eq!(bit, false),
    }
  }

  let arr: Vec<bool> = bits.iter().collect();
  assert_eq!(arr.len(), 8);
}

#[test]
fn can_convert_to_bytes_buffer() {
  let mut bits = Bitfield::new();

  assert_eq!(bits.to_bytes().unwrap(), vec![]);

  bits.set(0);

  let mut target = vec![0; 1024];
  target[0] = 128;
  assert_eq!(
    bits.to_bytes().unwrap(),
    target
  );

  bits.set(9000);

  let mut target2 = vec![0; 1024*2];
  target2[0] = 128;
  target2[9000/8] = 128;
  assert_eq!(
      bits.to_bytes().unwrap(),
      target2
    );
}
