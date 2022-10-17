#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    pac::Peripherals,
    prelude::*,
    spi::{Spi, SpiMode},
    timer::TimerGroup,
    Delay, Rtc, IO,
};
use esp_backtrace as _;
use esp_println::println;
use max7219_driver::{enums::DigitRowAddress, MAX7219};
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // GPIO2 (FSPIQ: MISO/SIO1)、
    // GPIO6 (FSPICLK)、GPIO7 (FSPID: MOSI/SIO0)、
    let sclk = io.pins.gpio6;
    let mosi = io.pins.gpio7;
    let cs = io.pins.gpio5.into_push_pull_output();

    let spi = Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sclk,
        mosi,
        800u32.Hz(),
        SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let mut max7219 = MAX7219::new(spi, cs).unwrap();

    // Initialize the Delay peripheral
    let mut delay = Delay::new(&clocks);

    // Application Code
    // Initialize Display
    max7219.init_display(true);

    println!("hello max7219!");
    loop {
        let mut data: u8 = 1;
        for addr in 1..9 {
            max7219.draw_row_or_digit(addr.try_into().unwrap(), data);
            data = data << 1;
            delay.delay_ms(500_u32);
        }

        // Clear the LED matrix row by row with 500ms delay in between
        for addr in 1..9 {
            max7219.draw_row_or_digit(DigitRowAddress::try_from(addr).unwrap(), data);
            delay.delay_ms(500_u32);
        }
    }
}
