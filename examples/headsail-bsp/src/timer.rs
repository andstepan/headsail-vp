use crate::{mmap::TIMER0_ADDR, write_u32, read_u32};

#[inline]
pub fn timer0_enable()
{
    // Read register
    let mut reg = read_u32(TIMER0_ADDR);
    // Make enable bit 1
    reg |= 0x1;
    // Write register back
    write_u32(TIMER0_ADDR, reg as u32);
}

#[inline]
pub fn timer0_disable()
{
    // Read register
    let mut reg = read_u32(TIMER0_ADDR);
    // Write 0 to bit 0 but all other bits untouched
    // !0x1 = 1111_1110 (example for 8 bits)
    // reg &= !0x1 => reg = reg & 1111_1110
    reg &= !0x1;
    // Write register back
    write_u32(TIMER0_ADDR, reg as u32);
}

#[inline]
pub fn timer0_get_count() -> u32 
{
    return read_u32(TIMER0_ADDR);
}
