[![crates.io](https://img.shields.io/crates/v/stm32-extras.svg)](https://crates.io/crates/stm32-extras)
[![crates.io](https://img.shields.io/crates/d/stm32-extras.svg)](https://crates.io/crates/stm32-extras)
[![CircleCI](https://img.shields.io/circleci/project/github/idubrov/stm32-extras.svg)](https://circleci.com/gh/idubrov/stm32-extras)

# stm32-extras

Extra API on top of STM32 device crates (`stm32f103xx`)

## Examples

Configuring GPIO pins without disturbing other pins (no read-modify-write which could lead to
data races):

```rust
use stm32_extras::GPIOExtras;
let gpioc = unsafe { &*stm32f103xx::GPIOC.get() }; // Get GPIOC somehow...

// Set pin to 2Mhz, open-drain.
// Modifies corresponding GPIO configuration bits without reads
gpioc.pin_config(13).output2().open_drain();
```

Generalized interface to GPIO pins:

```rust
use stm32_extras::GPIOExtras;
let gpioc = unsafe { &*stm32f103xx::GPIOC.get() }; // Get GPIOC somehow...

// Set pins 13, 14 and 15 on GPIOC to 1, 0 and 1.
gpioc.write_pin_range(13, 3, 0b101);
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
