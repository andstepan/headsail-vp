#![no_std]
#![no_main]

extern crate uart;

use hello_hpc::print_example_name;
use headsail_bsp::{rt::entry, sprintln};
use uart::{Uart, UartWriter};
use panic_halt as _;

const UART0_ADDR: usize = 0xFFF00000;

#[entry]
fn main() -> ! {
    print_example_name!();
    sprintln!("UART test start");

    // Construct the UART peripheral in Data mode
    let mut uart = unsafe { Uart::new(uart::UartAddress::Mmio(UART0_ADDR as *mut u8)) };
    let message = "Hello world using UART crate!\n";

    for byte in message.bytes()
    {
        uart.write_data(byte);
    }
    
    // Get the peripheral in Configuration mode
    let uart_conf = uart.configure_mode();

    // Get Baud rate from the peripheral and
    // translate it into human-readable format
    let baud_rate = uart_conf.get_baud();
    let baud_rate_u32 = get_baud_rate(baud_rate);

    // Switch the peripheral back into Data mode
    // and send Baud rate over serial
    let mut uart = uart_conf.data_mode();
    sprintln!("Initial Baud rate: {}", baud_rate_u32);

    // Switch the peripheral into configuration mode 
    // change the Baud rate to 115200
    let mut uart_conf = uart.configure_mode();
    uart_conf.set_baud(uart::Baud::B115200);

    // Get new Baud rate
    let baud_rate = uart_conf.get_baud();
    let baud_rate_u32 = get_baud_rate(baud_rate);

    // Switch the peripheral back into Data mode
    // and send new Baud rate over serial
    let mut uart = uart_conf.data_mode();
    sprintln!("Current Baud rate: {}", baud_rate_u32);

    // Get UartWriter
    let mut writer = UartWriter::new(uart);
    match writer {
        Some(_) => sprintln!("Sucessfully created UartWriter"),
        None => sprintln!("Failed to create writer"),
    };

    loop {}
}

fn get_baud_rate(x: uart::Baud) -> u32
{
    match x {
        uart::Baud::B115200 => 115_200,
        uart::Baud::B57600 => 57_600,
        uart::Baud::B38400 => 38_400,
        uart::Baud::B19200 => 19_200,
        uart::Baud::B9600 => 9_600,
        uart::Baud::B4800 => 4_800,
        uart::Baud::B2400 => 2_400,
        uart::Baud::B1200 => 1_200,
        uart::Baud::B300 => 300,
        uart::Baud::B50 => 50,
    }
}