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

const TIMER0_COUNTER_REG_OFFSET: usize = 0x8;
const TIMER0_CTRL_REG_OFFSET: usize = 0x0;

const TIMER0_ENABLE_BIT: u32 = 0b1;


#[inline]
pub fn timer0_enable()
{
    // Read register
    let mut reg = read_u32(TIMER0_BASE_ADDR + TIMER0_CTRL_REG_OFFSET);
    // Make enable bit 1
    reg |= TIMER0_ENABLE_BIT;
    // Write register back
    write_u32(TIMER0_BASE_ADDR + TIMER0_CTRL_REG_OFFSET, reg as u32);
}

#[inline]
pub fn timer0_disable()
{
    // Read register
    let mut reg = read_u32(TIMER0_BASE_ADDR + TIMER0_CTRL_REG_OFFSET);
    // Write 0 to bit 0 but all other bits untouched
    // !0x1 = 1111_1110 (example for 8 bits)
    // reg &= !0x1 => reg = reg & 1111_1110
    reg &= !TIMER0_ENABLE_BIT;
    // Write register back
    write_u32(TIMER0_BASE_ADDR + TIMER0_CTRL_REG_OFFSET, reg as u32);
}

#[inline]
pub fn timer0_get_count() -> u32 
{
    return read_u32(TIMER0_BASE_ADDR + TIMER0_COUNTER_REG_OFFSET);
}

#[inline]
pub fn timer0_get_ctrl_reg() -> u32
{
    return read_u32(TIMER0_BASE_ADDR + TIMER0_CTRL_REG_OFFSET);
}
