//! YD-CH582 Board
//! CH582M
#![no_std]
#![no_main]

use qingke_rt::highcode;
use ch58x_hal as hal;
use embedded_hal_1::delay::DelayNs;
use hal::gpio::{Input, Level, Output, OutputDrive, Pull};
use hal::peripherals;
use hal::rtc::{DateTime, Rtc};
use hal::uart::UartTx;

static mut SERIAL: Option<UartTx<peripherals::UART1>> = None;

macro_rules! println {
    ($($arg:tt)*) => {
        unsafe {
            use core::fmt::Write;
            use core::writeln;

            if let Some(uart) = SERIAL.as_mut() {
                writeln!(uart, $($arg)*).unwrap();
            }
        }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;

    let pa9 = unsafe { peripherals::PA9::steal() };
    let uart1 = unsafe { peripherals::UART1::steal() };
    let mut serial = UartTx::new(uart1, pa9, Default::default()).unwrap();

    let _ = writeln!(&mut serial, "\n\n\n{}", info);

    loop {}
}

#[qingke_rt::entry]
#[highcode]
fn main() -> ! {
    let mut config = hal::Config::default();
    config.clock.use_pll_60mhz().enable_lse();
    let p = hal::init(config);

    // GPIO
    let mut led = Output::new(p.PB4, Level::Low, OutputDrive::_5mA);
    let boot_btn = Input::new(p.PB22, Pull::Up);
    let rst_btn = Input::new(p.PB23, Pull::Up);

    let uart = UartTx::new(p.UART1, p.PA9, Default::default()).unwrap();
    unsafe {
        SERIAL.replace(uart);
    }

    let rtc = Rtc::new(p.RTC);

    println!("\n\nHello World!");
    println!("System Clocks: {}", hal::sysctl::clocks().hclk);
    println!("ChipID: 0x{:02x}", hal::signature::get_chip_id());
    println!("RTC datetime: {}", rtc.now());

    loop {
        led.toggle();
        println!("tick");

        hal::delay_ms(1000);

        println!("button: {} {}", boot_btn.is_low(), rst_btn.is_low());

        if rst_btn.is_low() {
            println!("button: {} {}", boot_btn.is_low(), rst_btn.is_low());
        }
    }
}
