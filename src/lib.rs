//! HAL layer for STM32 devices on top of STM32 device crates (`stm32f103xx`)
#![no_std]
#![feature(const_fn)]
#![feature(const_size_of)]
#![warn(missing_docs)]
#![deny(warnings)]


#[cfg(feature = "stm32f103xx")]
extern crate stm32f103xx;

/// HAL layer for GPIO ports. Includes:
/// * better API for configuring GPIO ports
/// * atomic updates to GPIO configuration and pins
///
/// # Examples
///
/// Configuring GPIO pins without disturbing other pins (no read-modify-write which could lead to
/// data races):
///
/// ```rust,no_run
/// # extern crate stm32_hal;
/// extern crate stm32f103xx;
/// use stm32f103xx::GPIOC;
/// use stm32_hal::gpio::Port;
///
/// # fn main() {
/// # let gpioc = unsafe { &*GPIOC.get() };
/// // Get GPIOC somehow...
/// // let gpioc = GPIOC.borrow(cs);
///
/// // Set pin to 2Mhz, open-drain.
/// // Modifies corresponding GPIO configuration bits without reads
/// gpioc.pin_config(13).output2().open_drain();
/// # }
/// ```
///
/// Generalized interface to GPIO pins:
///
/// ```rust,no_run
/// # extern crate stm32_hal;
/// # extern crate stm32f103xx;
/// use stm32f103xx::GPIOC;
/// use stm32_hal::gpio::Port;
///
/// # fn main() {
/// # let gpioc = unsafe { &*GPIOC.get() };
/// // Get GPIOC somehow...
/// // let gpioc = GPIOC.borrow(cs);
///
/// // Set pins 13, 14 and 15 on GPIOC to 1, 0 and 1.
/// gpioc.write_pin_range(13, 3, 0b101);
/// # }
/// ```
pub mod gpio;

/// Higher-level API for the Flash memory controller.
///
/// # Examples
///
/// Erasing flash memory page and writing some data to it:
///
/// ```rust,no_run
/// extern crate stm32f103xx;
/// extern crate stm32_hal;
/// use stm32f103xx::FLASH;
/// use stm32_hal::flash::Flash;
///
/// # pub fn main() {
/// # let flash = unsafe { &*FLASH.get() };
/// // Get flash somehow...
/// // let flash = FLASH.borrow(cs);
/// unsafe {
///     let flash = flash.unlock_guard().unwrap(); // Unlock Flash for writing
///     flash.erase_page(0x800_fc00).unwrap(); // last 1K page on a chip with 64K flash memory
///     flash.program_half_word(0x800_fc00, 0xcafe).unwrap();
///     flash.program_half_word(0x800_fc02, 0xbabe).unwrap();
/// }
/// # }
/// ```
///
pub mod flash;

#[cfg(feature = "stm32f103xx")]
mod bitband;