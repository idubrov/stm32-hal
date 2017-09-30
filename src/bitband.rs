const PERIPHERALS_BASE: usize = 0x4000_0000;
const PERIPHERALS_ALIAS: usize = 0x4200_0000;

pub fn to_bitband_address<S, T>(port: &T) -> &'static S {
    let byte_offset = (port as *const T as usize) - PERIPHERALS_BASE;
    let address = PERIPHERALS_ALIAS + byte_offset * 32;
    let ptr = address as *const S;
    unsafe { &*ptr }
}