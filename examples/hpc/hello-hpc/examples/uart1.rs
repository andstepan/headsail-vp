#![no_std]
#![no_main]

extern crate uart;
use hello_hpc::print_example_name;
use headsail_bsp::rt::entry;
use uart::Uart;
use panic_halt as _;

const UART0_ADDR: usize = 0xFFF00000;

#[entry]
fn main() -> ! {
    print_example_name!();
    let mut uart = unsafe { Uart::new(uart::UartAddress::Mmio(UART0_ADDR as *mut u8)) };
    
    let message = "Hello world using UART crate!";

    for byte in message.bytes()
    {
        uart.write_data(byte);
    }
    

    loop {}
}