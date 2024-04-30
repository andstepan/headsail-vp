/**
 * Date: 30/4/2024
 * Author: Andreas Stergiopoulos (andreas.stergiopoulos@tuni.fi)
 */

/** 
 * DISCLAIMER: THIS DRIVER IS **NOT** FOR THE APB_TIMER, WHICH
 * IS THE TIMER DESCRIBED IN THE IP DOCUMENTATION. IT IS INSTEAD 
 * FOR THE TIMER_UNIT PROVIDED BY THE PULP PLATFORM.
 * 
 * This happens because for the time being, the renode model for 
 * the APB timer is not available. If this changes, this driver 
 * should change too.
 */

use crate::{mmap::TIMER0_BASE_ADDR, write_u32, read_u32};

extern crate bit_field;
use bit_field::BitField;

// TODO: Ask Henri, implement correctly and REVERSE conditions
#[cfg(any(feature = "vp"))]
mod timer_interface {
    pub const TIMER0_CTRL_REG_OFFSET: usize = 0x0;
    pub const TIMER0_COUNTER_REG_OFFSET: usize = 0x8;
    pub const TIMER0_ENABLE_BIT: u32 = 0b0;
}

#[cfg(not(feature = "vp"))]
mod timer_interface {
    pub const TIMER0_COUNTER_REG_OFFSET: usize = 0x0;
    pub const TIMER0_CTRL_REG_OFFSET: usize = 0x4;
    pub const TIMER0_CMP_REG_OFFSET: usize = 0x8;
    pub const TIMER0_ENABLE_BIT: u32 = 0b0;
}


#[inline]
pub fn timer0_enable()
{
    use timer_interface::*;
    // Read register
    let mut reg = read_u32(TIMER0_BASE_ADDR + TIMER0_CTRL_REG_OFFSET);
    // Make enable bit 1
    reg.set_bit(TIMER0_ENABLE_BIT as usize, true);
    // Write register back
    write_u32(TIMER0_BASE_ADDR + TIMER0_CTRL_REG_OFFSET, reg as u32);
}

#[inline]
pub fn timer0_disable()
{
    use timer_interface::*;
    // Read register
    let mut reg = read_u32(TIMER0_BASE_ADDR + TIMER0_CTRL_REG_OFFSET);
    // Write 0 to bit 0 but leave all other bits untouched
    reg.set_bit(TIMER0_ENABLE_BIT as usize, false);
    // Write register back
    write_u32(TIMER0_BASE_ADDR + TIMER0_CTRL_REG_OFFSET, reg as u32);
}

#[inline]
#[cfg(debug_assertions)]
pub fn timer0_get_count() -> u32 
{
    use timer_interface::*;
    return read_u32(TIMER0_BASE_ADDR + TIMER0_COUNTER_REG_OFFSET);
}

#[inline]
#[cfg(debug_assertions)]
pub fn timer0_get_ctrl_reg() -> u32
{
    use timer_interface::*;
    return read_u32(TIMER0_BASE_ADDR + TIMER0_CTRL_REG_OFFSET);
}
