[![crates.io](https://img.shields.io/crates/v/stm32-bitband.svg)](https://crates.io/crates/stm32-bitband)
[![crates.io](https://img.shields.io/crates/d/stm32-bitband.svg)](https://crates.io/crates/stm32-bitband)
[![CircleCI](https://img.shields.io/circleci/project/github/idubrov/stm32-bitband.svg)](https://circleci.com/gh/idubrov/stm32-bitband)

# stm32-bitband

Bit-banding access to STM32 peripherals.

## Examples
```rust
use stm32_bitband::gpio_bitband;

let gpioc = unsafe { &*GPIOC.get() }; // Get GPIOC somehow...
let pin = gpio_bitband(gpioc).pin(13);
pin.output2();
pin.open_drain();
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
