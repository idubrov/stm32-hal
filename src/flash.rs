use core;
use core::result::Result;

#[cfg(feature = "stm32f103xx")]
use stm32f103xx::FLASH;

/// High-level API for the Flash memory
pub trait Flash where Self: Sized {
    /// Check if Flash program and erase controller is locked
    fn is_locked(&self) -> bool;

    /// Check Flash status
    fn status(&self) -> FlashResult;

    /// Unlocks the Flash program and erase controller (FPEC).
    ///
    /// # Panics
    /// Panics if unlock sequence did not unlock the flash
    unsafe fn unlock(&self);

    /// Lock the Flash program and erase controller (FPEC).
    unsafe fn lock(&self);

    /// Unlocks the Flash program and erase controller (FPEC).
    /// An RAII guard is returned to allow scoped unlock of the Flash. When the guard goes out of scope,
    /// the Flash will be unlocked.
    ///
    /// # Note
    /// If flash is unlocked already, unlock sequence is skipped and it Flash is not locked when guard drops.
    unsafe fn unlock_guard(&self) -> UnlockResult<Self> {
        let locked = self.is_locked();
        if locked {
            self.unlock();
        }
        Ok(UnlockGuard { flash: self, should_lock: locked })
    }

    /// Erase specified flash page. `address` must be an address of a beginning of the page in
    /// Flash memory.
    unsafe fn erase_page(&self, address: usize) -> FlashResult;

    /// Program half-word (16-bit) value at a specified address. `address` must be an address of
    /// a location in the Flash memory aligned to two bytes.
    unsafe fn program_half_word(&self, address: usize, data: u16) -> FlashResult;

    /// Erase all Flash pages
    unsafe fn erase_all_pages(&self) -> FlashResult;
}

/// Flash operation error
#[derive(Copy, Clone, Debug)]
pub enum FlashError {
    /// Flash program and erase controller failed to unlock
    UnlockFailed,
    /// Timeout while waiting for the completion of the operation
    Timeout,
    /// Address to be programmed contains a value different from '0xFFFF' before programming
    ProgrammingError,
    /// Programming a write-protected address of the Flash memory
    WriteProtectionError,
    /// Programming and erase controller is busy
    Busy
}

/// A type alias for the result of a Flash operation.
pub type FlashResult = Result<(), FlashError>;

/// A type alias for the result of a Flash unlock method.
pub type UnlockResult<'a, FlashT> = Result<UnlockGuard<'a, FlashT>, FlashError>;

/// An RAII implementation of a "scoped unlock" of a Flash. When this structure is dropped (falls
/// out of scope), the Flash will be locked.
pub struct UnlockGuard<'a, FlashT: Flash> where FlashT: 'a {
    flash: &'a FlashT,
    should_lock: bool
}

impl<'a, FlashT: Flash> Drop for UnlockGuard<'a, FlashT> {
    fn drop(&mut self) {
        if self.should_lock {
            unsafe {
                self.flash.lock();
            }
        }
    }
}

impl<'a, FlashT: Flash> core::ops::Deref for UnlockGuard<'a, FlashT> {
    type Target = FlashT;

    fn deref(&self) -> &FlashT {
        self.flash
    }
}


#[cfg(feature = "stm32f103xx")]
const FLASH_KEY1: u32 = 0x4567_0123;

#[cfg(feature = "stm32f103xx")]
const FLASH_KEY2: u32 = 0xCDEF_89AB;

#[cfg(feature = "stm32f103xx")]
const ERASE_TIMEOUT: u32 = 0x000B_0000;

#[cfg(feature = "stm32f103xx")]
const PROGRAM_TIMEOUT: u32 = 0x0000_2000;

#[cfg(feature = "stm32f103xx")]
impl Flash for FLASH {
    fn is_locked(&self) -> bool {
        self.cr.read().lock().bit_is_set()
    }

    fn status(&self) -> FlashResult {
        let sr = self.sr.read();
        if sr.bsy().bit_is_set() {
            Err(FlashError::Busy)
        } else if sr.pgerr().bit_is_set() {
            Err(FlashError::ProgrammingError)
        } else if sr.wrprterr().bit_is_set() {
            Err(FlashError::WriteProtectionError)
        } else {
            Ok(())
        }
    }

    unsafe fn erase_page(&self, address: usize) -> FlashResult {
        self.status()?;

        self.cr.modify(|_, w| w.per().set_bit());
        self.ar.write(|w| w.bits(address as u32));
        self.cr.modify(|_, w| w.strt().set_bit()); // Erase page
        let res = wait_complete(self, ERASE_TIMEOUT);
        self.cr.modify(|_, w| w.per().clear_bit());
        res
    }

    unsafe fn erase_all_pages(&self) -> FlashResult {
        self.status()?;

        self.cr.modify(|_, w| w.mer().set_bit());
        self.cr.modify(|_, w| w.strt().set_bit()); // Erase all pages
        let res = wait_complete(self, ERASE_TIMEOUT);
        self.cr.modify(|_, w| w.mer().clear_bit());
        res
    }

    unsafe fn program_half_word(&self, address: usize, data: u16) -> FlashResult {
        self.status()?;

        self.cr.modify(|_, w| w.pg().set_bit());
        core::ptr::write(address as *mut u16, data); // Program the half-word
        let res = wait_complete(self, PROGRAM_TIMEOUT);
        self.cr.modify(|_, w| w.pg().clear_bit());
        res
    }

    /// Unlocks the Flash program and erase controller (FPEC)
    unsafe fn unlock(&self) {
        self.keyr.write(|w| w.key().bits(FLASH_KEY1));
        self.keyr.write(|w| w.key().bits(FLASH_KEY2));
        if self.is_locked() {
            panic!("Flash should be unlocked at this point");
        }
    }

    /// Locks the Flash program and erase controller (FPEC)
    unsafe fn lock(&self) {
        self.cr.modify(|_, w| w.lock().set_bit());
    }
}

#[cfg(feature = "stm32f103xx")]
/// Wait till last Flash operation is complete and return Flash status.
fn wait_complete(flash: &FLASH, mut timeout: u32) -> FlashResult {
    while flash.sr.read().bsy().bit_is_set() && timeout > 0 {
        timeout -= 1
    }
    if timeout == 0 {
        return Err(FlashError::Timeout);
    }
    flash.status()
}
