# sparser-bitfield

__ARCHIVED__
There is already something similar: [ranges](https://docs.rs/ranges/).
This uses `Vec` internally, might be worth switching to `SkipList`,
maybe not worth it since for my use case the bitfields will mostly have
long continuous segments.

## Usage

```rust
extern crate sparser_bitfield;

use sparser_bitfield::Bitfield;

let mut bits = Bitfield::new();

bits.set(0);          // set first bit
bits.set(1);          // set second bit
bits.set(1_000_000);  // set the millionth bit

assert_eq!(bits.get(1), true);

bits.reset(1);
bits.reset(2_000);

assert_eq!(bits.get(1), false);
```

## License

[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)
