# sparse-bitfield

## Usage
```rust
extern crate sparse_bitfield;

use sparse_bitfield::Bitfield;

let mut bits = Bitfield::new(1024);
bits.set(0, true);          // set first bit
bits.set(1, true);          // set second bit
bits.set(1_000_000, true);  // set the millionth bit
assert!(bits.get(1));
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)
