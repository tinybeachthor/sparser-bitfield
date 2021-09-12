use proptest::proptest;
use rand::thread_rng;
use rand::seq::IteratorRandom;

use sparser_bitfield::{Bitfield, Change};

proptest! {
    #[test]
    fn single_bit(bit in 0usize..100_000_000) {
        let mut bits = Bitfield::new();
        assert_eq!(bits.set(bit), Change::Changed);
        assert_eq!(bits.get(bit), true);
        assert_eq!(bits.set(bit), Change::Unchanged);
        assert_eq!(bits.get(bit), true);
        assert_eq!(bits.set(bit), Change::Unchanged);
        assert_eq!(bits.get(bit), true);
    }

    #[test]
    fn two_bits(a in 0usize..10, b in 0usize..10) {
        let mut bits = Bitfield::new();

        assert_eq!(bits.set(a), Change::Changed);
        assert_eq!(bits.get(a), true);

        let change = bits.set(b);
        let expected_change = if a == b {
            Change::Unchanged
        }
        else {
            Change::Changed
        };
        assert_eq!(expected_change, change);

        assert_eq!(bits.get(b), true);
    }

    #[test]
    fn utilization(load in 0usize..100) {
        let size = 1_000;

        let mut rng = thread_rng();
        let count = (size / 100) * load;
        let sample = (0..size).choose_multiple(&mut rng, count);

        let mut bits = Bitfield::new();
        for i in sample.clone() {
            bits.set(i);
        }

        for i in sample {
            assert_eq!(bits.get(i), true);
        }
    }
}
