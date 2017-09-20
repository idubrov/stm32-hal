//! Bit-banding access to STM32 peripherals.
//! 
//! # Examples
//! ```rust,no_run
//! # struct FakeGpio;
//! # impl FakeGpio { pub fn get(&self) -> *mut u32 { std::ptr::null_mut() } }
//! # static GPIOC: FakeGpio = FakeGpio;
//! use stm32_bitband::gpio_bitband;
//!
//! let gpioc = unsafe { &*GPIOC.get() }; // Get GPIOC somehow...
//! let pin = gpio_bitband(gpioc).pin(13);
//! pin.output2();
//! pin.open_drain();
//! ```
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
extern crate vcell;
use vcell::VolatileCell;

const PERIPHERALS_BASE: usize = 0x4000_0000;
const PERIPHERALS_ALIAS: usize = 0x4200_0000;

/// Pin configuration registers
pub struct PinConfigBlock {
    mode_low: VolatileCell<u32>,
    mode_high: VolatileCell<u32>,
    cnf_low: VolatileCell<u32>,
    cnf_high: VolatileCell<u32>,
}

impl PinConfigBlock {
    /// Input mode (reset state)
    pub fn input(&self) {
        self.mode_low.set(0);
        self.mode_high.set(0);
    }

    /// Output mode, max speed 2 MHz.
    pub fn output2(&self) {
        self.mode_low.set(0);
        self.mode_high.set(1);
    }

    /// Output mode, max speed 10 MHz.
    pub fn output10(&self) {
        self.mode_low.set(1);
        self.mode_high.set(0);
    }

    /// Output mode, max speed 50 MHz.
    pub fn output50(&self) {
        self.mode_low.set(1);
        self.mode_high.set(1);
    }

    // Output config

    /// Push-pull
    pub fn push_pull(&self) {
        self.cnf_low.set(0);
    }

    /// Open-drain
    pub fn open_drain(&self) {
        self.cnf_low.set(1);
    }

    /// General purpose
    pub fn general(&self) {
        self.cnf_high.set(0);
    }

    /// Alternate function
    pub fn alternate(&self) {
        self.cnf_high.set(1);
    }

    // Input config

    /// Analog mode
    pub fn analog(&self) {
        self.cnf_low.set(0);
        self.cnf_high.set(0);
    }

    /// Floating input (reset state)
    pub fn floating(&self) {
        // Ordering is important: should never get reserved value of `11`
        self.cnf_high.set(0);
        self.cnf_low.set(1);
    }

    /// Input with pull-up / pull-down
    pub fn pull_up_down(&self) {
        self.cnf_low.set(0);
        self.cnf_high.set(1);
    }
}

/// GPIO port configuration bits
#[repr(C)]
pub struct RegisterBlock {
    pins: [PinConfigBlock; 16],
}

impl RegisterBlock {
    /// Get pin configuration bits
    pub fn pin(&self, pin: usize) -> &PinConfigBlock {
        &self.pins[pin]
    }
}

/// Convert reference from the GPIO peripheral (GPIOA-GPIOH) to the bit-banding variant
pub fn gpio_bitband<T>(port: &T) -> &RegisterBlock {
    let byte_offset = (port as *const T as usize) - PERIPHERALS_BASE;
    let address = PERIPHERALS_ALIAS + byte_offset * 32;
    let ptr = address as *const RegisterBlock;
    unsafe { &*ptr }
}